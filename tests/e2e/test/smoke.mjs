import path from "node:path";
import { fileURLToPath } from "node:url";
import { remote } from "webdriverio";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, "..", "..", "..");

const defaultExe = path.resolve(
  repoRoot,
  "target",
  "debug",
  "pdf-editor.exe"
);

const appPath = process.env.APP_EXE ?? defaultExe;
const appiumHost = process.env.APPIUM_HOST ?? "127.0.0.1";
const appiumPort = Number(process.env.APPIUM_PORT ?? "4723");

async function run() {
  const driver = await remote({
    hostname: appiumHost,
    port: appiumPort,
    path: "/",
    logLevel: "error",
    capabilities: {
      platformName: "Windows",
      "appium:automationName": "Windows",
      "appium:app": appPath,
      "appium:newCommandTimeout": 180
    }
  });

  try {
    // Basic launch smoke: window handle should become available quickly.
    await driver.pause(1500);
    const handle = await driver.getWindowHandle();
    if (!handle) {
      throw new Error("No window handle returned. App may have failed to launch.");
    }

    // Optional check: title should be non-empty after launch.
    const title = await driver.getTitle();
    if (typeof title !== "string") {
      throw new Error("Window title is not readable.");
    }

    console.log("E2E smoke passed. Window handle:", handle);
    console.log("Window title:", title);
  } finally {
    await driver.deleteSession();
  }
}

run().catch((err) => {
  console.error("E2E smoke failed:", err?.message ?? err);
  process.exit(1);
});
