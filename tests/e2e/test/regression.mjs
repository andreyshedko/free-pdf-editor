import path from "node:path";
import fs from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { remote } from "webdriverio";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, "..", "..", "..");

const defaultExe = path.resolve(repoRoot, "target", "debug", "pdf-editor.exe");

const appPath = process.env.APP_EXE ?? defaultExe;
const appiumHost = process.env.APPIUM_HOST ?? "127.0.0.1";
const appiumPort = Number(process.env.APPIUM_PORT ?? "4723");
const appDataRoot = process.env.APPDATA ?? "";

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function findByName(driver, name, timeout = 4000) {
  const started = Date.now();
  while (Date.now() - started < timeout) {
    const el = await driver.$(`name=${name}`);
    if (await el.isExisting()) {
      return el;
    }
    await sleep(150);
  }
  return null;
}

async function findByAnyName(driver, names, timeout = 4000) {
  const started = Date.now();
  while (Date.now() - started < timeout) {
    for (const name of names) {
      const el = await driver.$(`name=${name}`);
      if (await el.isExisting()) {
        return el;
      }
    }
    await sleep(150);
  }
  return null;
}

async function clickViewport(driver, x, y) {
  await driver.performActions([
    {
      type: "pointer",
      id: "mouse",
      parameters: { pointerType: "mouse" },
      actions: [
        { type: "pointerMove", duration: 0, x, y, origin: "viewport" },
        { type: "pointerDown", button: 0 },
        { type: "pointerUp", button: 0 }
      ]
    }
  ]);
  await driver.releaseActions();
}

async function findFirstEditControl(driver, timeout = 2500) {
  const started = Date.now();
  while (Date.now() - started < timeout) {
    const candidates = [
      await driver.$("xpath=(//Edit)[1]"),
      await driver.$("class name=Edit"),
      await driver.$("xpath=(//*[@ClassName='Edit'])[1]")
    ];

    for (const el of candidates) {
      if (await el.isExisting()) {
        return el;
      }
    }
    await sleep(120);
  }
  return null;
}

async function tryEnterSaveModeFromCanvasClick(driver) {
  const saveNames = ["Save", "Сохранить"];
  const existing = await findByAnyName(driver, saveNames, 600);
  if (existing) {
    return true;
  }

  const rect = await driver.getWindowRect();
  const probes = [
    [0.47, 0.34],
    [0.52, 0.34],
    [0.50, 0.38],
    [0.45, 0.42],
    [0.55, 0.42],
    [0.50, 0.46]
  ];

  for (const [rx, ry] of probes) {
    const x = Math.round(rect.width * rx);
    const y = Math.round(rect.height * ry);
    await clickViewport(driver, x, y);
    await sleep(500);
    const saveBtn = await findByAnyName(driver, saveNames, 900);
    if (saveBtn) {
      return true;
    }
  }

  return false;
}

function escapeXPathString(value) {
  if (!value.includes("'")) {
    return `'${value}'`;
  }
  if (!value.includes('"')) {
    return `"${value}"`;
  }
  const parts = value.split("'").map((p) => `'${p}'`);
  return `concat(${parts.join(", \"'\", ")})`;
}

async function findNameContains(driver, fragment, timeout = 4000) {
  const started = Date.now();
  const escaped = escapeXPathString(fragment);
  const query = `xpath=//*[contains(@Name, ${escaped})]`;
  while (Date.now() - started < timeout) {
    const el = await driver.$(query);
    if (await el.isExisting()) {
      return el;
    }
    await sleep(150);
  }
  return null;
}

function buildMinimalPdfBuffer() {
  const lines = [];
  const offsets = [0];
  let cursor = 0;

  const push = (line) => {
    lines.push(line);
    cursor += Buffer.byteLength(line, "ascii");
  };

  push("%PDF-1.4\n");

  const pushObject = (id, body) => {
    offsets[id] = cursor;
    push(`${id} 0 obj\n`);
    push(body);
    if (!body.endsWith("\n")) {
      push("\n");
    }
    push("endobj\n");
  };

  // Large centered text makes canvas hit-testing deterministic for e2e.
  const content = "BT\n/F1 72 Tf\n160 430 Td\n(E2E Fixture) Tj\nET\n";

  pushObject(1, "<< /Type /Catalog /Pages 2 0 R >>\n");
  pushObject(2, "<< /Type /Pages /Kids [3 0 R] /Count 1 >>\n");
  pushObject(
    3,
    "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>\n"
  );
  pushObject(4, `<< /Length ${Buffer.byteLength(content, "ascii")} >>\nstream\n${content}endstream\n`);
  pushObject(5, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\n");

  const xrefStart = cursor;
  push("xref\n");
  push("0 6\n");
  push("0000000000 65535 f \n");
  for (let id = 1; id <= 5; id += 1) {
    push(`${String(offsets[id]).padStart(10, "0")} 00000 n \n`);
  }
  push("trailer\n");
  push("<< /Size 6 /Root 1 0 R >>\n");
  push("startxref\n");
  push(`${xrefStart}\n`);
  push("%%EOF\n");

  return Buffer.from(lines.join(""), "ascii");
}

async function ensureFixturePdf() {
  const fixtureDir = path.resolve(repoRoot, "tests", "e2e", ".tmp");
  const fixturePath = path.resolve(fixtureDir, "e2e_sample.pdf");
  await fs.mkdir(fixtureDir, { recursive: true });
  await fs.writeFile(fixturePath, buildMinimalPdfBuffer());
  return fixturePath;
}

async function ensureRecentDocumentsFile(pdfPath) {
  if (!appDataRoot) {
    throw new Error("APPDATA is not set; cannot prepare recent documents fixture");
  }
  const appDir = path.resolve(appDataRoot, "free-pdf-editor");
  await fs.mkdir(appDir, { recursive: true });
  await fs.writeFile(path.resolve(appDir, "recent_documents.txt"), `${pdfPath}\n`, "utf8");
}

async function runScenario(name, fn) {
  try {
    await fn();
    console.log(`[PASS] ${name}`);
    return { name, ok: true };
  } catch (err) {
    console.error(`[FAIL] ${name}: ${err?.message ?? err}`);
    return { name, ok: false, error: err?.message ?? String(err) };
  }
}

async function run() {
  const fixturePdfPath = await ensureFixturePdf();
  await ensureRecentDocumentsFile(fixturePdfPath);

  const driver = await remote({
    hostname: appiumHost,
    port: appiumPort,
    path: "/",
    logLevel: "error",
    capabilities: {
      platformName: "Windows",
      "appium:automationName": "Windows",
      "appium:app": appPath,
      "appium:newCommandTimeout": 240
    }
  });

  try {
    await sleep(1500);

    const results = [];

    results.push(
      await runScenario("Open Recent submenu visible", async () => {
        const fileMenu = await findByAnyName(driver, ["File", "Файл"], 5000);
        if (!fileMenu) {
          throw new Error("File menu entry is not visible");
        }
        await fileMenu.click();

        const openRecent = await findByAnyName(driver, ["Open Recent", "Недавние файлы"], 5000);
        if (!openRecent) {
          throw new Error("Open Recent menu item not visible");
        }
        await openRecent.click();
      })
    );

    results.push(
      await runScenario("Open Recent first entry opens fixture", async () => {
        const fileMenu = await findByAnyName(driver, ["File", "Файл"], 5000);
        if (!fileMenu) {
          throw new Error("File menu entry is not visible");
        }
        await fileMenu.click();

        const openRecent = await findByAnyName(driver, ["Open Recent", "Недавние файлы"], 5000);
        if (!openRecent) {
          throw new Error("Open Recent menu item not visible");
        }
        await openRecent.click();

        const recentItem = await findNameContains(driver, "e2e_sample.pdf", 5000);
        if (!recentItem) {
          throw new Error("Fixture recent entry is not visible in submenu");
        }
        await recentItem.click();

        // Window should remain responsive and keep a valid handle after opening the recent entry.
        await sleep(900);
        const handle = await driver.getWindowHandle();
        if (!handle) {
          throw new Error("Window handle missing after opening fixture from Open Recent");
        }
      })
    );

    results.push(
      await runScenario("Clicking fixture text enters Save mode", async () => {
        const enteredSaveMode = await tryEnterSaveModeFromCanvasClick(driver);
        if (!enteredSaveMode) {
          throw new Error("Save action did not appear after repeated canvas clicks on fixture page");
        }
      })
    );

    results.push(
      await runScenario("Save mode persists after text save", async () => {
        const enteredSaveMode = await tryEnterSaveModeFromCanvasClick(driver);
        if (!enteredSaveMode) {
          throw new Error("Cannot start Save-mode persistence check because Save mode is not active");
        }

        const editor = await findFirstEditControl(driver, 2500);
        if (editor) {
          await editor.click();
          await editor.setValue("E2E Saved Text");
        }

        const saveBtn = await findByAnyName(driver, ["Save", "Сохранить"], 2000);
        if (!saveBtn) {
          throw new Error("Save button not found before commit action");
        }
        await saveBtn.click();
        await sleep(700);

        const saveStillVisible = await findByAnyName(driver, ["Save", "Сохранить"], 2200);
        if (!saveStillVisible) {
          throw new Error("Save mode did not persist after save action");
        }
      })
    );

    results.push(
      await runScenario("Open Recent remains stable across repeated open", async () => {
        for (let i = 0; i < 3; i += 1) {
          const fileMenu = await findByAnyName(driver, ["File", "Файл"], 5000);
          if (!fileMenu) {
            throw new Error(`File menu entry is not visible (iteration ${i + 1})`);
          }
          await fileMenu.click();

          const openRecent = await findByAnyName(driver, ["Open Recent", "Недавние файлы"], 5000);
          if (!openRecent) {
            throw new Error(`Open Recent menu item not visible (iteration ${i + 1})`);
          }
          await openRecent.click();

          const recentItem = await findNameContains(driver, "e2e_sample.pdf", 4000);
          if (!recentItem) {
            throw new Error(`Fixture entry disappeared in submenu (iteration ${i + 1})`);
          }
        }
      })
    );

    results.push(
      await runScenario("Text panel controls present (contract)", async () => {
        const closeBtn = await findByAnyName(driver, ["Close", "Закрыть"], 3000);
        if (!closeBtn) {
          throw new Error("Close button not found in text panel");
        }

        const saveOrInsert = await findByAnyName(driver, ["Save", "Insert", "Сохранить", "Вставить"], 3000);
        if (!saveOrInsert) {
          throw new Error("Neither Save nor Insert action button is discoverable");
        }
      })
    );

    const failed = results.filter((r) => !r.ok);
    if (failed.length > 0) {
      throw new Error(`${failed.length} scenario(s) failed`);
    }

    console.log("E2E regression passed.");
  } finally {
    await driver.deleteSession();
  }
}

run().catch((err) => {
  console.error("E2E regression failed:", err?.message ?? err);
  process.exit(1);
});
