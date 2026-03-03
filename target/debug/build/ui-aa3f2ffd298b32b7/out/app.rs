mod slint_generatedAppWindow {
     # ! [allow (non_snake_case , non_camel_case_types)] # ! [allow (unused_braces , unused_parens)] # ! [allow (clippy :: all , clippy :: pedantic , clippy :: nursery)] # ! [allow (unknown_lints , if_let_rescope , tail_expr_drop_order)] use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_15_1 = slint :: VersionCheck_1_15_1 ;
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerFluentPalette_57 {
         r#background : sp :: Property < slint :: Brush > , r#color_scheme : sp :: Property < sp :: r#ColorScheme > , r#dark_color_scheme : sp :: Property < bool > , r#foreground : sp :: Property < slint :: Brush > , globals : sp :: OnceCell < sp :: Weak < SharedGlobals >> , }
     impl InnerFluentPalette_57 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , globals : & sp :: Rc < SharedGlobals >) {
             # ! [allow (unused)] let _ = self . globals . set (sp :: Rc :: downgrade (globals)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#background }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                     . apply_pin (_self) . get () {
                         (sp :: Color :: from_argb_encoded ((4280032284f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294638330f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . color_scheme ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     ({
                         let r#tmp_FluentPalette_57_color_scheme = {
                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#color_scheme }
                         . apply_pin (_self) . get () ;
                         if ! (((r#tmp_FluentPalette_57_color_scheme . clone ())) == ((sp :: r#ColorScheme :: r#Unknown))) {
                             ((((r#tmp_FluentPalette_57_color_scheme . clone ())) == ((sp :: r#ColorScheme :: r#Dark)))) as _ }
                         else {
                             ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . color_scheme ())) == ((sp :: r#ColorScheme :: r#Dark)) }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#foreground }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                     . apply_pin (_self) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerFocusBorder_root_1 {
         r#root_1 : sp :: r#BasicBorderRectangle , r#rectangle_2 : sp :: r#BasicBorderRectangle , r#root_1_height : sp :: Property < sp :: LogicalLength > , r#root_1_width : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerFocusBorder_root_1 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerFocusBorder_root_1 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((3003121664f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . get () . get ()) as f64) - ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => ((((({
                     * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((4f64) as f64)) as sp :: Coord , (((({
                     * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((4f64) as f64)) as sp :: Coord , 2f64 as sp :: Coord , 2f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerButton_root_3 {
         r#root_3 : sp :: r#Empty , r#i_background_4 : sp :: r#BasicBorderRectangle , r#i_border_5 : sp :: r#BasicBorderRectangle , r#i_touch_area_11 : sp :: r#TouchArea , r#i_focus_scope_12 : sp :: r#FocusScope , r#root_3_checked : sp :: Property < bool > , r#root_3_has_focus : sp :: Property < bool > , r#root_3_height : sp :: Property < sp :: LogicalLength > , r#root_3_i_background_4_width : sp :: Property < sp :: LogicalLength > , r#root_3_i_layout_6_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_3_i_layout_6_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_3_i_layout_6_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_3_icon : sp :: Property < sp :: Image > , r#root_3_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_3_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_3_pressed : sp :: Property < bool > , r#root_3_state : sp :: Property < i32 > , r#root_3_text : sp :: Property < sp :: SharedString > , r#root_3_text_color : sp :: Property < slint :: Brush > , r#root_3_width : sp :: Property < sp :: LogicalLength > , r#root_3_x : sp :: Property < sp :: LogicalLength > , r#root_3_y : sp :: Property < sp :: LogicalLength > , r#root_3_accessible_action_default : sp :: Callback < () , () > , r#root_3_clicked : sp :: Callback < () , () > , repeater0 : sp :: Conditional < InnerComponent_image_7 > , repeater1 : sp :: Conditional < InnerComponent_text_9 > , repeater2 : sp :: Conditional < InnerComponent_focusborder_13 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerButton_root_3 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_3 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (((((((sp :: Image :: default () . size ()) . r#width) as f64) > ((0f64) as f64))) && (((((sp :: Image :: default () . size ()) . r#height) as f64) > ((0f64) as f64))))) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
                    ) . apply_pin (_self) . get ())) != ((sp :: SharedString :: from (""))))) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_has_focus }
                    ) . apply_pin (_self) . get ())) && ((({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ())))) as _ }
                 }
            ) ;
             sp :: Property :: link_two_way (({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self)) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_has_focus }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_background_4_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         4usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_3 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_7 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                             }
                         InnerButton_root_3 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         r#repeated_indices [2usize] = items_vec . len () as u32 ;
                         r#repeated_indices [2usize + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 12f64 as _ ;
                                 the_struct . r#end = 12f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_background_4_width }
                            ) . apply_pin (_self) . get () . get () as _ , r#spacing : 4f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_3 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_7 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                             }
                         InnerButton_root_3 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 4f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_3 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_7 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                             }
                         InnerButton_root_3 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 5f64 as _ ;
                             the_struct . r#end = 5f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_0 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_0 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_0 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                             the_struct . r#min_percent = (r#layout_info_0 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_0 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = 0f64 as _ ;
                             the_struct }
                         }
                    )) + (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + ((({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_1 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_1 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_1 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                             the_struct . r#min_percent = (r#layout_info_1 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_1 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = 0f64 as _ ;
                             the_struct }
                         }
                    )) + (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + ((({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_pressed }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ())) && ((({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_pressed }
                        ) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             if ({
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                                 (3f64) as _ }
                             else {
                                 if ({
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                                ) . apply_pin (_self) . get () {
                                     (4f64) as _ }
                                 else {
                                     0f64 }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_3_state = ({
                             * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_state }
                        ) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_3_state . clone () as f64) , & (1f64 as f64)) {
                             (if ({
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                            ) . apply_pin (_self) . get () {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((2281701375f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((1593835519f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1577058304f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_3_state . clone () as f64) , & (2f64 as f64)) {
                                 (if ({
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                                ) . apply_pin (_self) . get () {
                                     (slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((2147483648f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                    )) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_3_state . clone () as f64) , & (4f64 as f64)) {
                                     (slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                    )) as _ }
                                 else {
                                     if ({
                                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                                    ) . apply_pin (_self) . get () {
                                         (slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                        )) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                                        ) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_3_state = ({
                             * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_state }
                        ) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_3_state . clone () as f64) , & (1f64 as f64)) {
                             (if ({
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                            ) . apply_pin (_self) . get () {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((704643071f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((939524096f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((184549375f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_3_state . clone () as f64) , & (2f64 as f64)) {
                                 (if ({
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                                ) . apply_pin (_self) . get () {
                                     (slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((3428896255f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((3422576568f64) as u32) }
                                    )) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((150994943f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_3_state . clone () as f64) , & (3f64 as f64)) {
                                     (if ({
                                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                                    ) . apply_pin (_self) . get () {
                                         (slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((3865103871f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((3858784184f64) as u32) }
                                        )) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((2163866105f64) as u32) }
                                        ) }
                                    ) as _ }
                                 else {
                                     if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_3_state . clone () as f64) , & (4f64 as f64)) {
                                         (slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4284534271f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((4278214584f64) as u32) }
                                        )) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((268435455f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                        ) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_3_state = ({
                             * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_state }
                        ) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_3_state . clone () as f64) , & (1f64 as f64)) {
                             (if ({
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                            ) . apply_pin (_self) . get () {
                                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_3_state . clone () as f64) , & (2f64 as f64)) {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                                )) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_3_state . clone () as f64) , & (4f64 as f64)) {
                                     (if {
                                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9067000000000001f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((603979776f64) as u32) , position : 1f64 as _ }
                                        ]))) as _ }
                                     else {
                                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9067000000000001f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((1711276032f64) as u32) , position : 1f64 as _ }
                                        ])) }
                                    ) as _ }
                                 else {
                                     if {
                                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((402653183f64) as u32) , position : 0f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((301989888f64) as u32) , position : 0.0833f64 as _ }
                                        ]))) as _ }
                                     else {
                                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((251658240f64) as u32) , position : 0.9058f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((687865856f64) as u32) , position : 1f64 as _ }
                                        ])) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if false {
                                 ({
                                     ({
                                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                                    ) . apply_pin (_self) . set ((! ({
                                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                                    ) . apply_pin (_self) . get ()) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             ;
                             ({
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_clicked }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ! ((((((args . 0 . clone ()) . r#text)) == ((sp :: SharedString :: from (" "))))) || (((((args . 0 . clone ()) . r#text)) == ((sp :: SharedString :: from ("\n")))))) {
                                 ({
                                     {
                                         sp :: r#EventResult :: r#Reject }
                                     }
                                ) as _ }
                             else {
                                 {
                                     ({
                                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 }
                                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& ()) ;
                                     sp :: r#EventResult :: r#Accept }
                                 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_7 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                             * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                             * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_7 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_7 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 5u32 => (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Button , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
                ) . apply_pin (_self) . get ()) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#Default , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_7 {
         r#image_7 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_7 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_7 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _ = _self . parent . upgrade () . as_ref () . map (| x | ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_icon }
                ) . apply_pin (x . as_pin_ref ())) . map (| x | sp :: Property :: link_two_way (({
                     * & InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , x)) ;
                 }
             ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
                    ) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             ({
                 * & InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 20f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 20f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 20f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((((_self . parent . upgrade () . as_ref () . map (| x | ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
                ) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord , 20f64 as sp :: Coord , _self . parent . upgrade () . as_ref () . map (| x | ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layout_cache }
                ) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default () as sp :: Coord , 5f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_image_7 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_7 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_7) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_7 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_image_7 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_7 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_7 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 6u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_image_7 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_7 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_9 {
         r#text_9 : sp :: r#SimpleText , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_9 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_9 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text_color }
                    ) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((1.0766f64) as f64) * ((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (((400f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
                    ) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
                    ) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layout_cache }
                    ) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((((_self . parent . upgrade () . as_ref () . map (| x | ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
                ) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord , _self . parent . upgrade () . as_ref () . map (| x | ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layout_cache }
                ) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default () as sp :: Coord , _self . parent . upgrade () . as_ref () . map (| x | ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layout_cache }
                ) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default () as sp :: Coord , 5f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_text_9 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_9 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_9) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_9 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_text_9 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_9 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_9 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 7u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_9 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_9 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_focusborder_13 {
         r#focusborder_13 : InnerFocusBorder_root_1 , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_focusborder_13 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_focusborder_13 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerFocusBorder_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#focusborder_13 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             ({
                 InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 }
             + {
                 * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 }
                 + {
                     * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
                    ) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 }
                 + {
                     * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width }
                    ) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 }
             + {
                 * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 }
             + {
                 * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 }
             + {
                 * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerFocusBorder_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#focusborder_13 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 }
                 + {
                     * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_13 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_13 . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 }
                 + {
                     * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_13 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_13 . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (_self . parent . upgrade () . as_ref () . map (| x | ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
                ) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , _self . parent . upgrade () . as_ref () . map (| x | ({
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width }
                ) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 ..= 1u32 => return {
                     * & Self :: FIELD_OFFSETS . r#focusborder_13 }
                 . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_13 }
                 . apply_pin (_self) . accessible_role (0) , 1u32 ..= 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_13 }
                 . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_13 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_13 }
                 . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_13 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_13 }
                 . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_13 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_13 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_13 }
                 . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_focusborder_13 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_focusborder_13 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 }
             + {
                 * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 }
             + {
                 * & InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_focusborder_13) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_focusborder_13 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_focusborder_13 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_focusborder_13 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_focusborder_13 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 4u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_focusborder_13 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_focusborder_13 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerAppWindow {
         r#root_15 : sp :: r#WindowItem , r#empty_17 : sp :: r#Empty , r#empty_24 : sp :: r#Empty , r#flickable_25 : sp :: r#Flickable , r#flickable_viewport_26 : sp :: r#Empty , r#image_27 : sp :: r#ImageItem , r#vertical_bar_visibility_28 : sp :: r#Clip , r#vertical_bar_29 : sp :: r#BasicBorderRectangle , r#vertical_bar_clip_30 : sp :: r#Clip , r#thumb_31 : sp :: r#BasicBorderRectangle , r#touch_area_32 : sp :: r#TouchArea , r#up_scroll_button_Opacity_33 : sp :: r#Opacity , r#up_scroll_button_34 : sp :: r#TouchArea , r#icon_Opacity_35 : sp :: r#Opacity , r#icon_36 : sp :: r#ImageItem , r#down_scroll_button_Opacity_37 : sp :: r#Opacity , r#down_scroll_button_38 : sp :: r#TouchArea , r#icon_Opacity_39 : sp :: r#Opacity , r#icon_40 : sp :: r#ImageItem , r#horizontal_bar_visibility_41 : sp :: r#Clip , r#horizontal_bar_42 : sp :: r#BasicBorderRectangle , r#horizontal_bar_clip_43 : sp :: r#Clip , r#thumb_44 : sp :: r#BasicBorderRectangle , r#touch_area_45 : sp :: r#TouchArea , r#up_scroll_button_Opacity_46 : sp :: r#Opacity , r#up_scroll_button_47 : sp :: r#TouchArea , r#icon_Opacity_48 : sp :: r#Opacity , r#icon_49 : sp :: r#ImageItem , r#down_scroll_button_Opacity_50 : sp :: r#Opacity , r#down_scroll_button_51 : sp :: r#TouchArea , r#icon_Opacity_52 : sp :: r#Opacity , r#icon_53 : sp :: r#ImageItem , r#rectangle_54 : sp :: r#Rectangle , r#text_55 : sp :: r#SimpleText , r#button_18 : InnerButton_root_3 , r#button_19 : InnerButton_root_3 , r#button_20 : InnerButton_root_3 , r#button_21 : InnerButton_root_3 , r#button_22 : InnerButton_root_3 , r#button_23 : InnerButton_root_3 , r#root_15_current_page : sp :: Property < i32 > , r#root_15_down_scroll_button_38_state : sp :: Property < i32 > , r#root_15_down_scroll_button_51_state : sp :: Property < i32 > , r#root_15_empty_16_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_15_empty_16_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_15_empty_16_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_15_empty_17_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_15_empty_17_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_15_empty_17_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_15_empty_24_horizontal_scrollbar_policy : sp :: Property < sp :: r#ScrollBarPolicy > , r#root_15_empty_24_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_15_empty_24_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_15_empty_24_vertical_scrollbar_policy : sp :: Property < sp :: r#ScrollBarPolicy > , r#root_15_empty_24_visible_height : sp :: Property < sp :: LogicalLength > , r#root_15_empty_24_visible_width : sp :: Property < sp :: LogicalLength > , r#root_15_flickable_25_horizontal_stretch : sp :: Property < f32 > , r#root_15_flickable_25_max_height : sp :: Property < sp :: LogicalLength > , r#root_15_flickable_25_max_width : sp :: Property < sp :: LogicalLength > , r#root_15_flickable_25_min_height : sp :: Property < sp :: LogicalLength > , r#root_15_flickable_25_min_width : sp :: Property < sp :: LogicalLength > , r#root_15_flickable_25_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_15_flickable_25_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_15_flickable_25_vertical_stretch : sp :: Property < f32 > , r#root_15_horizontal_bar_42_maximum : sp :: Property < sp :: LogicalLength > , r#root_15_horizontal_bar_42_size : sp :: Property < sp :: LogicalLength > , r#root_15_horizontal_bar_42_state : sp :: Property < i32 > , r#root_15_horizontal_bar_42_visible : sp :: Property < bool > , r#root_15_horizontal_bar_42_width : sp :: Property < sp :: LogicalLength > , r#root_15_image_27_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_15_image_27_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_15_image_27_x : sp :: Property < sp :: LogicalLength > , r#root_15_image_27_y : sp :: Property < sp :: LogicalLength > , r#root_15_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_15_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_15_page_count : sp :: Property < i32 > , r#root_15_page_image : sp :: Property < sp :: Image > , r#root_15_status_text : sp :: Property < sp :: SharedString > , r#root_15_text_55_min_height : sp :: Property < sp :: LogicalLength > , r#root_15_text_55_min_width : sp :: Property < sp :: LogicalLength > , r#root_15_text_55_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_15_text_55_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_15_thumb_31_height : sp :: Property < sp :: LogicalLength > , r#root_15_thumb_31_width : sp :: Property < sp :: LogicalLength > , r#root_15_thumb_31_y : sp :: Property < sp :: LogicalLength > , r#root_15_thumb_44_height : sp :: Property < sp :: LogicalLength > , r#root_15_thumb_44_width : sp :: Property < sp :: LogicalLength > , r#root_15_thumb_44_x : sp :: Property < sp :: LogicalLength > , r#root_15_touch_area_32_saved_values : sp :: Property < (sp :: Coord , sp :: Coord , sp :: Coord , sp :: Coord ,) > , r#root_15_touch_area_45_saved_values : sp :: Property < (sp :: Coord , sp :: Coord , sp :: Coord , sp :: Coord ,) > , r#root_15_up_scroll_button_34_state : sp :: Property < i32 > , r#root_15_up_scroll_button_47_state : sp :: Property < i32 > , r#root_15_vertical_bar_29_height : sp :: Property < sp :: LogicalLength > , r#root_15_vertical_bar_29_maximum : sp :: Property < sp :: LogicalLength > , r#root_15_vertical_bar_29_size : sp :: Property < sp :: LogicalLength > , r#root_15_vertical_bar_29_state : sp :: Property < i32 > , r#root_15_vertical_bar_29_visible : sp :: Property < bool > , r#root_15_horizontal_bar_42_scrolled : sp :: Callback < () , () > , r#root_15_next_page : sp :: Callback < () , () > , r#root_15_open_document : sp :: Callback < () , () > , r#root_15_prev_page : sp :: Callback < () , () > , r#root_15_save_document : sp :: Callback < () , () > , r#root_15_vertical_bar_29_scrolled : sp :: Callback < () , () > , r#root_15_zoom_in : sp :: Callback < () , () > , r#root_15_zoom_out : sp :: Callback < () , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerAppWindow >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerAppWindow {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerButton_root_3 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_18 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 4u32 - 1 , tree_index_of_first_child + 10u32 - 1) ;
             InnerButton_root_3 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_19 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 5u32 - 1 , tree_index_of_first_child + 17u32 - 1) ;
             InnerButton_root_3 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_20 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 6u32 - 1 , tree_index_of_first_child + 24u32 - 1) ;
             InnerButton_root_3 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_21 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 7u32 - 1 , tree_index_of_first_child + 31u32 - 1) ;
             InnerButton_root_3 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_22 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 8u32 - 1 , tree_index_of_first_child + 38u32 - 1) ;
             InnerButton_root_3 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_23 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 9u32 - 1 , tree_index_of_first_child + 45u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#background }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_current_page }
            ) . apply_pin (_self) . set ({
                 (((0f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_down_scroll_button_38_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_38 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_38 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_down_scroll_button_51_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_51 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_51 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 40f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 40f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 50f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_54 }
                                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 24f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 24f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 8f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_54 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 40f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 40f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_54 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 24f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 24f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
                                     + {
                                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
                                     + {
                                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
                                     + {
                                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
                                     + {
                                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
                                     + {
                                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
                                     + {
                                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 4f64 as _ ;
                             the_struct . r#end = 4f64 as _ ;
                             the_struct }
                         as _ , r#size : (((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((8f64) as f64)) as f64) - ((8f64) as f64)) as _ , r#spacing : 4f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
                             + {
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
                             + {
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
                             + {
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
                             + {
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
                             + {
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
                             + {
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 4f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 4f64 as _ ;
                         the_struct . r#end = 4f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
                             + {
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
                             + {
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
                             + {
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
                             + {
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
                             + {
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
                             + {
                                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
                                 + {
                                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 4f64 as _ ;
                         the_struct . r#end = 4f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_horizontal_scrollbar_policy }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#ScrollBarPolicy :: r#AsNeeded) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    )))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    )))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_vertical_scrollbar_policy }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#ScrollBarPolicy :: r#AsNeeded) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_visible_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_visible_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((8f64) as f64)) as f64) - ((8f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 52u32 - 1))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 52u32 - 1))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 52u32 - 1))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 52u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 52u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 52u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 52u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_flickable_25_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 52u32 - 1))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get ()) as f64) - ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_visible_width }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_scrolled }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#flicked) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (6f64) as _ }
                     else {
                         2f64 }
                     as sp :: Coord)) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_45 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) || ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_51 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())))) || ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_47 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_15_empty_24_horizontal_scrollbar_policy = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_horizontal_scrollbar_policy }
                        ) . apply_pin (_self) . get () ;
                         ((((r#tmp_root_15_empty_24_horizontal_scrollbar_policy . clone ())) == ((sp :: r#ScrollBarPolicy :: r#AlwaysOn)))) || ((((((r#tmp_root_15_empty_24_horizontal_scrollbar_policy . clone ())) == ((sp :: r#ScrollBarPolicy :: r#AsNeeded)))) && ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_maximum }
                        ) . apply_pin (_self) . get () . get ()) as f64) > ((0f64) as f64))))) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_visible }
                    ) . apply_pin (_self) . get () {
                         ((((((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((8f64) as f64)) as f64) - ((8f64) as f64)) as f64) - ((14f64) as f64))) as _ }
                     else {
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((8f64) as f64)) as f64) - ((8f64) as f64) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_image_27_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#image_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 56u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_image_27_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#image_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 56u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_image_27_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get ()) as f64) - (((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_image_27_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (0f64 as sp :: Coord)) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_image_27_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get ()) as f64) - (((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_image_27_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (0f64 as sp :: Coord)) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_2 = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_2 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_2 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = 800f64 as _ ;
                             the_struct . r#min_percent = (r#layout_info_2 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_2 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_2 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    )) + ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_3 = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_3 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_3 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = 600f64 as _ ;
                             the_struct . r#min_percent = (r#layout_info_3 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_3 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_3 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    )) + ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_page_count }
            ) . apply_pin (_self) . set ({
                 (((0f64) as i32)) as i32 }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_status_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Ready")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_text_55_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 81u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_text_55_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 81u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_text_55_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 81u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_text_55_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 81u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_31_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_root_15_vertical_bar_29_maximum = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_maximum }
                        ) . apply_pin (_self) . get () . get () ;
                         ((if ((r#tmp_root_15_vertical_bar_29_maximum . clone ()) as f64) <= ((((0f64) as f64) / ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64)) as f64) {
                             (0f64) as _ }
                         else {
                             {
                                 let r#tmp_root_15_vertical_bar_29_page_size = ({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_visible_height }
                                ) . apply_pin (_self) . get () . get () ;
                                 ((((16f64 as sp :: Coord) . min (({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_height }
                                ) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord) . max ((((((({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_height }
                                ) . apply_pin (_self) . get () . get ()) as f64) - ((32f64) as f64)) as f64) * ((((r#tmp_root_15_vertical_bar_29_page_size . clone ()) as f64) / ((((r#tmp_root_15_vertical_bar_29_maximum . clone ()) as f64) + ((r#tmp_root_15_vertical_bar_29_page_size . clone ()) as f64)) as f64)) as f64)) as sp :: Coord)) as f64) * ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64) }
                             }
                        ) as f64) / ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64) }
                     as sp :: Coord)) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_31_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_size }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_31_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((16f64) as f64) + ((((((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_height }
                    ) . apply_pin (_self) . get () . get ()) as f64) - ((32f64) as f64)) as f64) - ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_31_height }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((- ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) / ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_maximum }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_44_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_size }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_44_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_root_15_horizontal_bar_42_maximum = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_maximum }
                        ) . apply_pin (_self) . get () . get () ;
                         ((if ((r#tmp_root_15_horizontal_bar_42_maximum . clone ()) as f64) <= ((((0f64) as f64) / ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64)) as f64) {
                             (0f64) as _ }
                         else {
                             {
                                 let r#tmp_root_15_horizontal_bar_42_page_size = ({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_visible_width }
                                ) . apply_pin (_self) . get () . get () ;
                                 ((((16f64 as sp :: Coord) . min (({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_width }
                                ) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord) . max ((((((((({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_width }
                                ) . apply_pin (_self) . get () . get ()) as f64) - ((32f64) as f64)) as f64) * ((r#tmp_root_15_horizontal_bar_42_page_size . clone ()) as f64)) as f64) / ((((r#tmp_root_15_horizontal_bar_42_maximum . clone ()) as f64) + ((r#tmp_root_15_horizontal_bar_42_page_size . clone ()) as f64)) as f64)) as sp :: Coord)) as f64) * ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64) }
                             }
                        ) as f64) / ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64) }
                     as sp :: Coord)) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_44_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((16f64) as f64) + ((((((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_width }
                    ) . apply_pin (_self) . get () . get ()) as f64) - ((32f64) as f64)) as f64) - ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_44_width }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((- ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) / ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_maximum }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Free PDF Editor")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_up_scroll_button_34_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_34 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_34 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_up_scroll_button_47_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_47 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_47 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_visible }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache }
                        ) . apply_pin (_self) . get () [3usize]) as f64) - ((14f64) as f64))) as _ }
                     else {
                         ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache }
                        ) . apply_pin (_self) . get () [3usize] }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get ()) as f64) - ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_visible_height }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_scrolled }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#flicked) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (6f64) as _ }
                     else {
                         2f64 }
                     as sp :: Coord)) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_32 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) || ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_38 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())))) || ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_34 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_15_empty_24_vertical_scrollbar_policy = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_vertical_scrollbar_policy }
                        ) . apply_pin (_self) . get () ;
                         ((((r#tmp_root_15_empty_24_vertical_scrollbar_policy . clone ())) == ((sp :: r#ScrollBarPolicy :: r#AlwaysOn)))) || ((((((r#tmp_root_15_empty_24_vertical_scrollbar_policy . clone ())) == ((sp :: r#ScrollBarPolicy :: r#AsNeeded)))) && ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_maximum }
                        ) . apply_pin (_self) . get () . get ()) as f64) > ((0f64) as f64))))) }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_open_document }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (32f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Open")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_save_document }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (32f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Save")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_zoom_in }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (32f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Zoom In")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                    ) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                    ) . apply_pin (_self) . get () [4usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_zoom_out }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (32f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Zoom Out")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                    ) . apply_pin (_self) . get () [7usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                    ) . apply_pin (_self) . get () [6usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_prev_page }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (32f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("◀")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                    ) . apply_pin (_self) . get () [9usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                    ) . apply_pin (_self) . get () [8usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_next_page }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (32f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("▶")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                    ) . apply_pin (_self) . get () [11usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                    ) . apply_pin (_self) . get () [10usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_visible_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_visible_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#image_27 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_image_27_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (0f64 as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#image_27 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#image_27 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_page_image }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#image_27 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_image_27_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (0f64 as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_28 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_visible }
                    ) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_29 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4281084972f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((4293980400f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_29 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_29 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_clip_30 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_clip_30 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_clip_30 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_clip_30 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_clip_30 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_clip_30 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#thumb_31 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#thumb_31 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_31_width }
                    ) . apply_pin (_self) . get () . get ()) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_32 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_32 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true)) && ((({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_32 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 ({
                                     if ! sp :: ApproxEq :: < f64 > :: approx_eq (& ((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_touch_area_32_saved_values }
                                    ) . apply_pin (_self) . get ()) . 0 as f64) , & (({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_maximum }
                                    ) . apply_pin (_self) . get () . get () as f64)) {
                                         ({
                                             _self . r#fn_touch_area_32_update_saved_values () }
                                        ) ;
                                         }
                                     else {
                                         {
                                             }
                                         }
                                     ;
                                     ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- (0f64 as sp :: Coord) . max ((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min (((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_touch_area_32_saved_values }
                                    ) . apply_pin (_self) . get ()) . 1) as f64) + ((if false {
                                         ((((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_32 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get ()) as f64) - (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_touch_area_32_saved_values }
                                        ) . apply_pin (_self) . get ()) . 2) as f64)) as f64) * ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as f64) / ((((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((32f64) as f64)) as f64) - ((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_31_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64))) as _ }
                                     else {
                                         ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_32 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get ()) as f64) - (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_touch_area_32_saved_values }
                                        ) . apply_pin (_self) . get ()) . 3) as f64)) as f64) * ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as f64) / ((((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((32f64) as f64)) as f64) - ((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_31_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64) }
                                    ) as f64)) as sp :: Coord) as sp :: Coord)) as sp :: Coord) as _) ;
                                     ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_scrolled }
                                    ) . apply_pin (_self) . call (& ()) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_32 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button)) == ((sp :: r#PointerEventButton :: r#Left)))) && (((((args . 0 . clone ()) . r#kind)) == ((sp :: r#PointerEventKind :: r#Down)))) {
                                 ({
                                     _self . r#fn_touch_area_32_update_saved_values () }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_32 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression0 = {
                                 let r#return_check_merge0 = if ((false)) && ((! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_x as f64) , & (0f64 as f64)))) {
                                     ((false , {
                                         ({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((0f64 as sp :: Coord) . min ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) + (((args . 0 . clone ()) . r#delta_x) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,)) as _ }
                                 else {
                                     if ! (((! false)) && ((! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_y as f64) , & (0f64 as f64))))) {
                                         ((true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                     else {
                                         (false , {
                                             ({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_maximum }
                                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) + (((args . 0 . clone ()) . r#delta_y) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,) }
                                     }
                                 ;
                                 if (r#return_check_merge0 . clone ()) . 0 {
                                     (({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                 else {
                                     (sp :: r#EventResult :: r#Reject , false , (r#return_check_merge0 . clone ()) . 1 ,) }
                                 }
                             ;
                             if (r#returned_expression0 . clone ()) . 1 {
                                 ((r#returned_expression0 . clone ()) . 0) as _ }
                             else {
                                 (r#returned_expression0 . clone ()) . 2 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_Opacity_33 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_34 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min ((((({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) + ((10f64) as f64)) as sp :: Coord) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_34 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_Opacity_35 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_up_scroll_button_34_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        ) }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         (((r#image_implicit_size . clone ()) . r#height) as f64) / (((r#image_implicit_size . clone ()) . r#width) as f64) }
                    ) as f64) * ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_up_scroll_button_34_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (6f64) as _ }
                     else {
                         8f64 }
                     as sp :: Coord)) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_Opacity_37 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_38 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_maximum }
                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((((({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as sp :: Coord) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_38 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_Opacity_39 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_down_scroll_button_38_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        ) }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         (((r#image_implicit_size . clone ()) . r#height) as f64) / (((r#image_implicit_size . clone ()) . r#width) as f64) }
                    ) as f64) * ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_down_scroll_button_38_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (6f64) as _ }
                     else {
                         8f64 }
                     as sp :: Coord)) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_41 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_visible }
                    ) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_42 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4281084972f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((4293980400f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_42 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_42 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_43 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_43 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_43 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_43 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_43 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_43 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#thumb_44 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#thumb_44 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_44_height }
                    ) . apply_pin (_self) . get () . get ()) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_45 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_45 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true)) && ((({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_45 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 ({
                                     if ! sp :: ApproxEq :: < f64 > :: approx_eq (& ((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_touch_area_45_saved_values }
                                    ) . apply_pin (_self) . get ()) . 0 as f64) , & (({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_maximum }
                                    ) . apply_pin (_self) . get () . get () as f64)) {
                                         ({
                                             _self . r#fn_touch_area_45_update_saved_values () }
                                        ) ;
                                         }
                                     else {
                                         {
                                             }
                                         }
                                     ;
                                     ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- (0f64 as sp :: Coord) . max ((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min (((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_touch_area_45_saved_values }
                                    ) . apply_pin (_self) . get ()) . 1) as f64) + ((if true {
                                         ((((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_45 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get ()) as f64) - (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_touch_area_45_saved_values }
                                        ) . apply_pin (_self) . get ()) . 2) as f64)) as f64) * ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as f64) / ((((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((32f64) as f64)) as f64) - ((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_44_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64))) as _ }
                                     else {
                                         ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_45 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get ()) as f64) - (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_touch_area_45_saved_values }
                                        ) . apply_pin (_self) . get ()) . 3) as f64)) as f64) * ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as f64) / ((((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((32f64) as f64)) as f64) - ((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_44_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64) }
                                    ) as f64)) as sp :: Coord) as sp :: Coord)) as sp :: Coord) as _) ;
                                     ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_scrolled }
                                    ) . apply_pin (_self) . call (& ()) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_45 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button)) == ((sp :: r#PointerEventButton :: r#Left)))) && (((((args . 0 . clone ()) . r#kind)) == ((sp :: r#PointerEventKind :: r#Down)))) {
                                 ({
                                     _self . r#fn_touch_area_45_update_saved_values () }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_45 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression1 = {
                                 let r#return_check_merge1 = if ((true)) && ((! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_x as f64) , & (0f64 as f64)))) {
                                     ((false , {
                                         ({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((0f64 as sp :: Coord) . min ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) + (((args . 0 . clone ()) . r#delta_x) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,)) as _ }
                                 else {
                                     if ! (((! true)) && ((! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_y as f64) , & (0f64 as f64))))) {
                                         ((true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                     else {
                                         (false , {
                                             ({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_maximum }
                                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) + (((args . 0 . clone ()) . r#delta_y) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,) }
                                     }
                                 ;
                                 if (r#return_check_merge1 . clone ()) . 0 {
                                     (({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                 else {
                                     (sp :: r#EventResult :: r#Reject , false , (r#return_check_merge1 . clone ()) . 1 ,) }
                                 }
                             ;
                             if (r#returned_expression1 . clone ()) . 1 {
                                 ((r#returned_expression1 . clone ()) . 0) as _ }
                             else {
                                 (r#returned_expression1 . clone ()) . 2 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_Opacity_46 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_47 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min ((((({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) + ((10f64) as f64)) as sp :: Coord) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_47 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_Opacity_48 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_up_scroll_button_47_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        ) }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         (((r#image_implicit_size . clone ()) . r#height) as f64) / (((r#image_implicit_size . clone ()) . r#width) as f64) }
                    ) as f64) * ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_up_scroll_button_47_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (4f64) as _ }
                     else {
                         6f64 }
                     as sp :: Coord)) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_Opacity_50 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_51 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_maximum }
                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((((({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as sp :: Coord) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_51 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_Opacity_52 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_down_scroll_button_51_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#dark_color_scheme }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        ) }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         (((r#image_implicit_size . clone ()) . r#height) as f64) / (((r#image_implicit_size . clone ()) . r#width) as f64) }
                    ) as f64) * ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_down_scroll_button_51_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (4f64) as _ }
                     else {
                         6f64 }
                     as sp :: Coord)) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = 0f64 as _ ;
                         the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                         the_struct . r#duration = 150f64 as _ ;
                         the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                         the_struct . r#iteration_count = 1f64 as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_54 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4293454056f64) as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerFluentPalette_57 :: FIELD_OFFSETS . r#foreground }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_57 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_text_55_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_text_55_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_status_text }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_text_55_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_text_55_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#resize_border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#image_27 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#image_27 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#image_27 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_28 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_28 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_28 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_28 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_28 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_29 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_29 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_29 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_clip_30 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_clip_30 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_clip_30 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_clip_30 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_clip_30 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#thumb_31 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#thumb_31 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_32 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_32 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_34 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_34 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_38 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_38 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_41 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_41 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_41 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_41 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_41 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_42 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_42 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_42 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_43 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_43 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_43 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_43 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_43 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#thumb_44 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#thumb_44 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_45 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_45 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_47 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_47 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_51 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_51 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_54 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerButton_root_3 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_18 }
             . apply_pin (x)) ,) ;
             InnerButton_root_3 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_19 }
             . apply_pin (x)) ,) ;
             InnerButton_root_3 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_20 }
             . apply_pin (x)) ,) ;
             InnerButton_root_3 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_21 }
             . apply_pin (x)) ,) ;
             InnerButton_root_3 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_22 }
             . apply_pin (x)) ,) ;
             InnerButton_root_3 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_23 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 {
                     {
                         }
                     ;
                     {
                         }
                     }
                 ;
                 {
                     {
                         }
                     ;
                     {
                         }
                     }
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_18 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 3u32 ..= 5u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_19 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 3u32 , order , visitor) }
                 6u32 ..= 8u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_20 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 6u32 , order , visitor) }
                 9u32 ..= 11u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_21 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 9u32 , order , visitor) }
                 12u32 ..= 14u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_22 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 12u32 , order , visitor) }
                 15u32 ..= 17u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_23 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 15u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 800f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 600f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_18 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 3u32 ..= 5u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_19 }
                     . apply_pin (_self) . subtree_range (dyn_index - 3u32) }
                 6u32 ..= 8u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_20 }
                     . apply_pin (_self) . subtree_range (dyn_index - 6u32) }
                 9u32 ..= 11u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_21 }
                     . apply_pin (_self) . subtree_range (dyn_index - 9u32) }
                 12u32 ..= 14u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_22 }
                     . apply_pin (_self) . subtree_range (dyn_index - 12u32) }
                 15u32 ..= 17u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_23 }
                     . apply_pin (_self) . subtree_range (dyn_index - 15u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_18 }
                     . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 3u32 ..= 5u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_19 }
                     . apply_pin (_self) . subtree_component (dyn_index - 3u32 , subtree_index , result) }
                 6u32 ..= 8u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_20 }
                     . apply_pin (_self) . subtree_component (dyn_index - 6u32 , subtree_index , result) }
                 9u32 ..= 11u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_21 }
                     . apply_pin (_self) . subtree_component (dyn_index - 9u32 , subtree_index , result) }
                 12u32 ..= 14u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_22 }
                     . apply_pin (_self) . subtree_component (dyn_index - 12u32 , subtree_index , result) }
                 15u32 ..= 17u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_23 }
                     . apply_pin (_self) . subtree_component (dyn_index - 15u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => (40f64 as sp :: Coord , (((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((8f64) as f64)) as f64) - ((8f64) as f64)) as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , (((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((8f64) as f64)) as f64) - ((8f64) as f64)) as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 3u32 => (24f64 as sp :: Coord , (((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((8f64) as f64)) as f64) - ((8f64) as f64)) as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord ,) , 4u32 => (32f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 4f64 as sp :: Coord ,) , 5u32 => (32f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 4f64 as sp :: Coord ,) , 6u32 => (32f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , 4f64 as sp :: Coord ,) , 7u32 => (32f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [7usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [6usize] as sp :: Coord , 4f64 as sp :: Coord ,) , 8u32 => (32f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [9usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [8usize] as sp :: Coord , 4f64 as sp :: Coord ,) , 9u32 => (32f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [11usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [10usize] as sp :: Coord , 4f64 as sp :: Coord ,) , 52u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_visible_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_visible_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 53u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 54u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 55u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 56u32 => ((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_image_27_preferred_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (0f64 as sp :: Coord) as sp :: Coord , (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_image_27_preferred_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (0f64 as sp :: Coord) as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_image_27_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_image_27_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 57u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 14f64 as sp :: Coord , (((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_visible_width }
                ) . apply_pin (_self) . get () . get ()) as f64) + ((0f64) as f64)) as f64) - ((14f64) as f64)) as sp :: Coord , 0f64 as sp :: Coord ,) , 58u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 14f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 59u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_31_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_31_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , (((10f64) as f64) - ((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_31_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_31_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 60u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 14f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 61u32 => (6f64 as sp :: Coord , 8f64 as sp :: Coord , 3f64 as sp :: Coord , 4f64 as sp :: Coord ,) , 62u32 => (6f64 as sp :: Coord , 8f64 as sp :: Coord , 3f64 as sp :: Coord , (((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((6f64) as f64)) as f64) - ((4f64) as f64)) as sp :: Coord ,) , 63u32 => (6f64 as sp :: Coord , 8f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 64u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (((((8f64) as f64) - ((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord , (((((6f64) as f64) - ((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , 65u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 66u32 => (6f64 as sp :: Coord , 8f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 67u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (((((8f64) as f64) - ((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord , (((((6f64) as f64) - ((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , 68u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 69u32 => (14f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , (((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_24_visible_height }
                ) . apply_pin (_self) . get () . get ()) as f64) + ((0f64) as f64)) as f64) - ((14f64) as f64)) as sp :: Coord ,) , 70u32 => (14f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 71u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_44_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_44_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_44_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , (((10f64) as f64) - ((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_thumb_44_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord ,) , 72u32 => (14f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 73u32 => (8f64 as sp :: Coord , 6f64 as sp :: Coord , 4f64 as sp :: Coord , 3f64 as sp :: Coord ,) , 74u32 => (8f64 as sp :: Coord , 6f64 as sp :: Coord , (((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((6f64) as f64)) as f64) - ((4f64) as f64)) as sp :: Coord , 3f64 as sp :: Coord ,) , 75u32 => (8f64 as sp :: Coord , 6f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 76u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (((((6f64) as f64) - ((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord , (((((8f64) as f64) - ((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , 77u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 78u32 => (8f64 as sp :: Coord , 6f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 79u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (((((6f64) as f64) - ((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord , (((((8f64) as f64) - ((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , 80u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 81u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 8f64 as sp :: Coord , (((((24f64) as f64) - ((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , 10u32 ..= 16u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_18 }
                 . apply_pin (_self) . item_geometry (index - 10u32 + 1) , 17u32 ..= 23u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_19 }
                 . apply_pin (_self) . item_geometry (index - 17u32 + 1) , 24u32 ..= 30u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_20 }
                 . apply_pin (_self) . item_geometry (index - 24u32 + 1) , 31u32 ..= 37u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_21 }
                 . apply_pin (_self) . item_geometry (index - 31u32 + 1) , 38u32 ..= 44u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . item_geometry (index - 38u32 + 1) , 45u32 ..= 51u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_23 }
                 . apply_pin (_self) . item_geometry (index - 45u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 4u32 => sp :: r#AccessibleRole :: r#Button , 5u32 => sp :: r#AccessibleRole :: r#Button , 6u32 => sp :: r#AccessibleRole :: r#Button , 7u32 => sp :: r#AccessibleRole :: r#Button , 8u32 => sp :: r#AccessibleRole :: r#Button , 9u32 => sp :: r#AccessibleRole :: r#Button , 56u32 => sp :: r#AccessibleRole :: r#Image , 81u32 => sp :: r#AccessibleRole :: r#Text , 4u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_18 }
                 . apply_pin (_self) . accessible_role (0) , 10u32 ..= 16u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_18 }
                 . apply_pin (_self) . accessible_role (index - 10u32 + 1) , 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_19 }
                 . apply_pin (_self) . accessible_role (0) , 17u32 ..= 23u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_19 }
                 . apply_pin (_self) . accessible_role (index - 17u32 + 1) , 6u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_20 }
                 . apply_pin (_self) . accessible_role (0) , 24u32 ..= 30u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_20 }
                 . apply_pin (_self) . accessible_role (index - 24u32 + 1) , 7u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_21 }
                 . apply_pin (_self) . accessible_role (0) , 31u32 ..= 37u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_21 }
                 . apply_pin (_self) . accessible_role (index - 31u32 + 1) , 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . accessible_role (0) , 38u32 ..= 44u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . accessible_role (index - 38u32 + 1) , 9u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_23 }
                 . apply_pin (_self) . accessible_role (0) , 45u32 ..= 51u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_23 }
                 . apply_pin (_self) . accessible_role (index - 45u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (4u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (4u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (4u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (4u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
                ) . apply_pin (_self) . get ()) , (5u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (5u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (5u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (5u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
                ) . apply_pin (_self) . get ()) , (6u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (6u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (6u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (6u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
                ) . apply_pin (_self) . get ()) , (7u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (7u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (7u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (7u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
                ) . apply_pin (_self) . get ()) , (8u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (8u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (8u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (8u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
                ) . apply_pin (_self) . get ()) , (9u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (9u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (9u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (9u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
                 + {
                     * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text }
                ) . apply_pin (_self) . get ()) , (81u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_status_text }
                ) . apply_pin (_self) . get ()) , (4u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_18 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (10u32 ..= 16u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_18 }
                 . apply_pin (_self) . accessible_string_property (index - 10u32 + 1 , what) , (5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_19 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (17u32 ..= 23u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_19 }
                 . apply_pin (_self) . accessible_string_property (index - 17u32 + 1 , what) , (6u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_20 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (24u32 ..= 30u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_20 }
                 . apply_pin (_self) . accessible_string_property (index - 24u32 + 1 , what) , (7u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_21 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (31u32 ..= 37u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_21 }
                 . apply_pin (_self) . accessible_string_property (index - 31u32 + 1 , what) , (8u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (38u32 ..= 44u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . accessible_string_property (index - 38u32 + 1 , what) , (9u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_23 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (45u32 ..= 51u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_23 }
                 . apply_pin (_self) . accessible_string_property (index - 45u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (4u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
                     + {
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (5u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
                     + {
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (6u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
                     + {
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (7u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
                     + {
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (8u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
                     + {
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (9u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
                     + {
                         * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (4u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_18 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (10u32 ..= 16u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_18 }
                 . apply_pin (_self) . accessibility_action (index - 10u32 + 1 , action) , (5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_19 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (17u32 ..= 23u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_19 }
                 . apply_pin (_self) . accessibility_action (index - 17u32 + 1 , action) , (6u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_20 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (24u32 ..= 30u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_20 }
                 . apply_pin (_self) . accessibility_action (index - 24u32 + 1 , action) , (7u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_21 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (31u32 ..= 37u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_21 }
                 . apply_pin (_self) . accessibility_action (index - 31u32 + 1 , action) , (8u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (38u32 ..= 44u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . accessibility_action (index - 38u32 + 1 , action) , (9u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_23 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (45u32 ..= 51u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_23 }
                 . apply_pin (_self) . accessibility_action (index - 45u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 4u32 => sp :: SupportedAccessibilityAction :: r#Default , 5u32 => sp :: SupportedAccessibilityAction :: r#Default , 6u32 => sp :: SupportedAccessibilityAction :: r#Default , 7u32 => sp :: SupportedAccessibilityAction :: r#Default , 8u32 => sp :: SupportedAccessibilityAction :: r#Default , 9u32 => sp :: SupportedAccessibilityAction :: r#Default , 4u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_18 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 10u32 ..= 16u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_18 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 10u32 + 1) , 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_19 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 17u32 ..= 23u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_19 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 17u32 + 1) , 6u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_20 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 24u32 ..= 30u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_20 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 24u32 + 1) , 7u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_21 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 31u32 ..= 37u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_21 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 31u32 + 1) , 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 38u32 ..= 44u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 38u32 + 1) , 9u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_23 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 45u32 ..= 51u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_23 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 45u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 10u32 ..= 16u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_18 }
                 . apply_pin (_self) . item_element_infos (index - 10u32 + 1) , 17u32 ..= 23u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_19 }
                 . apply_pin (_self) . item_element_infos (index - 17u32 + 1) , 24u32 ..= 30u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_20 }
                 . apply_pin (_self) . item_element_infos (index - 24u32 + 1) , 31u32 ..= 37u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_21 }
                 . apply_pin (_self) . item_element_infos (index - 31u32 + 1) , 38u32 ..= 44u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . item_element_infos (index - 38u32 + 1) , 45u32 ..= 51u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_23 }
                 . apply_pin (_self) . item_element_infos (index - 45u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_touch_area_32_update_saved_values (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_touch_area_32_saved_values }
            ) . apply_pin (_self) . set ((({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_vertical_bar_29_maximum }
            ) . apply_pin (_self) . get () . get () as sp :: Coord , (- ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as sp :: Coord , ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_32 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_32 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as sp :: Coord ,) as _)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_touch_area_45_update_saved_values (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_touch_area_45_saved_values }
            ) . apply_pin (_self) . set ((({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_horizontal_bar_42_maximum }
            ) . apply_pin (_self) . get () . get () as sp :: Coord , (- ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as sp :: Coord , ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_45 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_45 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as sp :: Coord ,) as _)) ;
             }
         }
     impl InnerAppWindow {
         fn new () -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = SharedGlobals :: new (sp :: VRc :: downgrade (& self_dyn_rc)) ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             82usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 6u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 52u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 81u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 10u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 17u32 , parent_index : 1u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 24u32 , parent_index : 1u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 31u32 , parent_index : 1u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 38u32 , parent_index : 1u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 45u32 , parent_index : 1u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 14u32 , parent_index : 4u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 17u32 , parent_index : 4u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 17u32 , parent_index : 4u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 17u32 , parent_index : 10u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 10u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 21u32 , parent_index : 5u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 24u32 , parent_index : 5u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 24u32 , parent_index : 5u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 5u32 , parent_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 24u32 , parent_index : 17u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 3u32 , parent_index : 17u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 4u32 , parent_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 28u32 , parent_index : 6u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 31u32 , parent_index : 6u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 31u32 , parent_index : 6u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 8u32 , parent_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 31u32 , parent_index : 24u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 6u32 , parent_index : 24u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 7u32 , parent_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 35u32 , parent_index : 7u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 38u32 , parent_index : 7u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 38u32 , parent_index : 7u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 11u32 , parent_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 38u32 , parent_index : 31u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 9u32 , parent_index : 31u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 10u32 , parent_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 42u32 , parent_index : 8u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 45u32 , parent_index : 8u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 45u32 , parent_index : 8u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 14u32 , parent_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 45u32 , parent_index : 38u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 12u32 , parent_index : 38u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 13u32 , parent_index : 38u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 49u32 , parent_index : 9u32 , item_array_index : 30u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 52u32 , parent_index : 9u32 , item_array_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 52u32 , parent_index : 9u32 , item_array_index : 32u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 17u32 , parent_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 52u32 , parent_index : 45u32 , item_array_index : 33u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 15u32 , parent_index : 45u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 16u32 , parent_index : 45u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 55u32 , parent_index : 2u32 , item_array_index : 34u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 57u32 , parent_index : 2u32 , item_array_index : 35u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 69u32 , parent_index : 2u32 , item_array_index : 36u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 56u32 , parent_index : 52u32 , item_array_index : 37u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 57u32 , parent_index : 55u32 , item_array_index : 38u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 58u32 , parent_index : 53u32 , item_array_index : 39u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 59u32 , parent_index : 57u32 , item_array_index : 40u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 63u32 , parent_index : 58u32 , item_array_index : 41u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 63u32 , parent_index : 58u32 , item_array_index : 42u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 63u32 , parent_index : 58u32 , item_array_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 66u32 , parent_index : 58u32 , item_array_index : 44u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 64u32 , parent_index : 61u32 , item_array_index : 45u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 65u32 , parent_index : 63u32 , item_array_index : 46u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 66u32 , parent_index : 64u32 , item_array_index : 47u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 67u32 , parent_index : 62u32 , item_array_index : 48u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 68u32 , parent_index : 66u32 , item_array_index : 49u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 69u32 , parent_index : 67u32 , item_array_index : 50u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 70u32 , parent_index : 54u32 , item_array_index : 51u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 71u32 , parent_index : 69u32 , item_array_index : 52u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 75u32 , parent_index : 70u32 , item_array_index : 53u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 75u32 , parent_index : 70u32 , item_array_index : 54u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 75u32 , parent_index : 70u32 , item_array_index : 55u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 78u32 , parent_index : 70u32 , item_array_index : 56u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 76u32 , parent_index : 73u32 , item_array_index : 57u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 77u32 , parent_index : 75u32 , item_array_index : 58u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 78u32 , parent_index : 76u32 , item_array_index : 59u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 79u32 , parent_index : 74u32 , item_array_index : 60u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 80u32 , parent_index : 78u32 , item_array_index : 61u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 81u32 , parent_index : 79u32 , item_array_index : 62u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 82u32 , parent_index : 3u32 , item_array_index : 63u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerAppWindow , sp :: ItemVTable , sp :: AllowPin > ;
             64usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_17 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_24 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_54 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#root_3 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_18 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_19 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_20 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_21 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_23 }
             + {
                 * & InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_25 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_28 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_41 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#flickable_viewport_26 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#image_27 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_29 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#vertical_bar_clip_30 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#thumb_31 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_32 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_Opacity_33 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_Opacity_37 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_34 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_Opacity_35 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_36 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_38 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_Opacity_39 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_40 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_42 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_43 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#thumb_44 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#touch_area_45 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_Opacity_46 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_Opacity_50 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#up_scroll_button_47 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_Opacity_48 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_49 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#down_scroll_button_51 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_Opacity_52 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#icon_53 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_55 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerAppWindow) ;
         }
     ;
     impl sp :: PinnedDrop for InnerAppWindow {
         fn drop (self : :: core :: pin :: Pin < & mut InnerAppWindow >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerAppWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerAppWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#AppWindow (sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) ;
     impl r#AppWindow {
         pub fn new () -> :: core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerAppWindow :: new () ? ;
             inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             InnerAppWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn get_current_page (& self) -> i32 {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_current_page }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_current_page (& self , value : i32) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_current_page }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_next_page (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_next_page }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_next_page (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_next_page }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_open_document (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_open_document }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_open_document (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_open_document }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_page_count (& self) -> i32 {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_page_count }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_page_count (& self , value : i32) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_page_count }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_page_image (& self) -> sp :: Image {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_page_image }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_page_image (& self , value : sp :: Image) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_page_image }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_prev_page (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_prev_page }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_prev_page (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_prev_page }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_save_document (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_save_document }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_save_document (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_save_document }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_status_text (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_status_text }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_status_text (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_status_text }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_zoom_in (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_zoom_in }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_zoom_in (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_zoom_in }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_zoom_out (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_zoom_out }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_zoom_out (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_15_zoom_out }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         }
     impl From < r#AppWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > {
         fn from (value : r#AppWindow) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#AppWindow {
         type WeakInner = sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow > ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (sp :: VRc :: downgrade (& self . 0)) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn upgrade_from_weak_inner (inner : & Self :: WeakInner) -> sp :: Option < Self > {
             sp :: Some (Self (inner . upgrade () ?)) }
         fn run (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             :: core :: result :: Result :: Ok (()) }
         fn show (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     struct SharedGlobals {
         global_FluentPalette_57 : :: core :: pin :: Pin < sp :: Rc < InnerFluentPalette_57 >> , window_adapter : sp :: OnceCell < sp :: WindowAdapterRc > , root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable > , }
     impl SharedGlobals {
         fn new (root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable >) -> sp :: Rc < Self > {
             let _self = sp :: Rc :: new (Self {
                 global_FluentPalette_57 : InnerFluentPalette_57 :: new () , window_adapter : :: core :: default :: Default :: default () , root_item_tree_weak , }
            ) ;
             _self . global_FluentPalette_57 . clone () . init (& _self) ;
             _self }
         fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
                 :: core :: result :: Result :: Ok (adapter) }
            ) }
         fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter . get () . cloned () }
         }
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = b"<svg width=\"8\" height=\"6\" viewBox=\"0 0 8 6\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 1C0 0.864583 0.0260417 0.735677 0.078125 0.613281C0.130208 0.490885 0.200521 0.385417 0.289062 0.296875C0.380208 0.205729 0.485677 0.134115 0.605469 0.0820312C0.72526 0.0273438 0.854167 0 0.992188 0H7.01172C7.14714 0 7.27474 0.0260417 7.39453 0.078125C7.51693 0.130208 7.6224 0.201823 7.71094 0.292969C7.79948 0.384115 7.86979 0.490885 7.92188 0.613281C7.97396 0.733073 8 0.860677 8 0.996094C8 1.10547 7.98438 1.20573 7.95312 1.29688C7.92448 1.38802 7.88021 1.47917 7.82031 1.57031L5.21875 5.35547C5.08073 5.55599 4.90365 5.71354 4.6875 5.82812C4.47396 5.94271 4.24479 6 4 6C3.75521 6 3.52474 5.94271 3.30859 5.82812C3.09505 5.71354 2.91927 5.55599 2.78125 5.35547L0.179688 1.57031C0.119792 1.48177 0.0742188 1.39193 0.0429688 1.30078C0.0143229 1.20964 0 1.10938 0 1Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_3 : & 'static [u8] = b"<svg width=\"6\" height=\"8\" viewBox=\"0 0 6 8\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 4C0 3.75521 0.0572917 3.52604 0.171875 3.3125C0.286458 3.09635 0.44401 2.91927 0.644531 2.78125L4.42969 0.179687C4.51823 0.119792 4.60807 0.0755208 4.69922 0.046875C4.79036 0.015625 4.89062 0 5 0C5.13542 0 5.26432 0.0260417 5.38672 0.078125C5.50911 0.130208 5.61458 0.201823 5.70312 0.292969C5.79427 0.38151 5.86589 0.485677 5.91797 0.605469C5.97266 0.72526 6 0.854167 6 0.992187L6 7.00781C6 7.14583 5.97266 7.27474 5.91797 7.39453C5.86589 7.51432 5.79427 7.61979 5.70313 7.71094C5.61458 7.79948 5.50911 7.86979 5.38672 7.92187C5.26432 7.97396 5.13542 8 5 8C4.79427 8 4.60417 7.9401 4.42969 7.82031L0.644531 5.21875C0.44401 5.08073 0.286458 4.90495 0.171875 4.69141C0.0572917 4.47526 0 4.24479 0 4Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_1 : & 'static [u8] = b"<svg width=\"6\" height=\"8\" viewBox=\"0 0 6 8\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 7.00781L0 0.992187C0 0.854167 0.0260417 0.72526 0.078125 0.605469C0.132812 0.485677 0.204427 0.38151 0.292969 0.292969C0.384115 0.201823 0.489583 0.130208 0.609375 0.078125C0.731771 0.0260417 0.861979 0 1 0C1.20573 0 1.39583 0.0598958 1.57031 0.179687L5.35547 2.78125C5.55859 2.92187 5.71615 3.09896 5.82813 3.3125C5.94271 3.52604 6 3.75521 6 4C6 4.24479 5.94271 4.47396 5.82813 4.6875C5.71615 4.90104 5.55859 5.07812 5.35547 5.21875L1.57031 7.82031C1.39583 7.9401 1.20573 8 1 8C0.861979 8 0.731771 7.97396 0.609375 7.92188C0.489583 7.86979 0.384115 7.79948 0.292969 7.71094C0.204427 7.61979 0.132813 7.51432 0.078125 7.39453C0.0260417 7.27474 0 7.14583 0 7.00781Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_2 : & 'static [u8] = b"<svg width=\"8\" height=\"6\" viewBox=\"0 0 8 6\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0.992188 6C0.854167 6 0.72526 5.97396 0.605469 5.92188C0.485677 5.86719 0.380208 5.79557 0.289062 5.70703C0.200521 5.61589 0.130208 5.51042 0.078125 5.39062C0.0260417 5.26823 0 5.13802 0 5C0 4.89062 0.0143229 4.79036 0.0429688 4.69922C0.0742188 4.60807 0.119792 4.51823 0.179688 4.42969L2.78125 0.644531C2.84896 0.545573 2.92839 0.458333 3.01953 0.382812C3.11068 0.307292 3.20833 0.244792 3.3125 0.195312C3.41927 0.143229 3.53125 0.104167 3.64844 0.078125C3.76562 0.0520833 3.88281 0.0390625 4 0.0390625C4.11719 0.0390625 4.23438 0.0520833 4.35156 0.078125C4.46875 0.104167 4.57943 0.143229 4.68359 0.195312C4.79036 0.244792 4.88932 0.307292 4.98047 0.382812C5.07161 0.458333 5.15104 0.545573 5.21875 0.644531L7.82031 4.42969C7.88021 4.51823 7.92448 4.60807 7.95312 4.69922C7.98438 4.79036 8 4.89062 8 5C8 5.13802 7.97396 5.26823 7.92188 5.39062C7.86979 5.51042 7.79948 5.61589 7.71094 5.70703C7.6224 5.79557 7.51693 5.86719 7.39453 5.92188C7.27474 5.97396 7.14714 6 7.01172 6H0.992188Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     }
 # [allow (unused_imports)] pub use slint_generatedAppWindow :: {
     r#AppWindow , }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
