use std::{cell::RefCell, ffi::{CStr, CString, c_char, c_void}};
use std::panic::{catch_unwind, AssertUnwindSafe};

pub type JObject = *mut c_void;
#[repr(C)]
pub struct HookParam {
    pub this_ptr: JObject,
    pub args: *mut JObject,
    pub args_count: u32,
    pub padding: u32,
    pub result: JObject,
    pub return_early: u8,
    pub has_result: u8,
    pub reserved: u16,
    pub padding2: u32,
}
#[repr(C)]
pub struct PluginMetadata {
    pub name: *const c_char,
    pub version: *const c_char,
    pub author: *const c_char,
    pub description: *const c_char,
}

unsafe impl Sync for PluginMetadata {}
pub type HookBeforeFn = Option<unsafe extern "C" fn(*mut HookParam)>;
pub type HookAfterFn = HookBeforeFn;
pub type ComposableFn = Option<unsafe extern "C" fn(JObject, usize)>;
pub type ScopedComposableFn = Option<unsafe extern "C" fn(JObject, JObject, usize)>;
pub type UpdateHookFn = Option<unsafe extern "C" fn(*const c_char, usize) -> bool>;
pub type MessageBeforeSendFn = Option<unsafe extern "C" fn(i64, *const c_char, usize) -> *const c_char>;
pub type MessageAfterSendFn = Option<unsafe extern "C" fn(i64, i64, usize)>;
pub type MessageReceiveFn = Option<unsafe extern "C" fn(i64, i64, *const c_char, usize)>;
pub type ChatActionClickFn = Option<unsafe extern "C" fn(i64, usize)>;
#[repr(C)]
pub struct PluginApi {
    pub register_hook: Option<unsafe extern "C" fn(class_name: *const c_char, method_name: *const c_char, method_descriptor: *const c_char, before_cb: HookBeforeFn, after_cb: HookAfterFn) -> i32>,
    pub log_info: Option<unsafe extern "C" fn(tag: *const c_char, msg: *const c_char)>,
    pub log_error: Option<unsafe extern "C" fn(tag: *const c_char, msg: *const c_char)>,
    pub get_storage_dir: Option<unsafe extern "C" fn(plugin_name: *const c_char, out_buf: *mut c_char, max_len: u32) -> i32>,
    pub get_cache_dir: Option<unsafe extern "C" fn(plugin_name: *const c_char, out_buf: *mut c_char, max_len: u32) -> i32>,
    pub resolve_asset: Option<unsafe extern "C" fn(plugin_name: *const c_char, asset_name: *const c_char, out_buf: *mut c_char, max_len: u32) -> i32>,
    pub get_current_user_id: Option<unsafe extern "C" fn() -> i64>,
    pub get_active_chat_id: Option<unsafe extern "C" fn() -> i64>,
    pub execute_tdlib_query: Option<unsafe extern "C" fn(json_query: *const c_char, out_buf: *mut c_char, max_len: u32) -> i32>,
    pub create_composable_lambda: Option<unsafe extern "C" fn(cb: ComposableFn, user_data: usize) -> JObject>,
    pub create_scoped_lambda: Option<unsafe extern "C" fn(cb: ScopedComposableFn, user_data: usize) -> JObject>,
    pub replace_composable: Option<unsafe extern "C" fn(param: *mut HookParam, cb: ComposableFn, user_data: usize)>,
    pub composer_start_group: Option<unsafe extern "C" fn(composer: JObject, key: i32)>,
    pub composer_end_group: Option<unsafe extern "C" fn(composer: JObject)>,
    pub composer_get_remembered: Option<unsafe extern "C" fn(composer: JObject) -> JObject>,
    pub composer_set_remembered: Option<unsafe extern "C" fn(composer: JObject, val: JObject)>,
    pub composer_is_skipping: Option<unsafe extern "C" fn(composer: JObject) -> bool>,
    pub register_update_hook: Option<unsafe extern "C" fn(cb: UpdateHookFn, user_data: usize) -> i32>,
    pub register_message_send_hook: Option<unsafe extern "C" fn(before_cb: MessageBeforeSendFn, after_cb: MessageAfterSendFn, user_data: usize) -> i32>,
    pub register_message_receive_hook: Option<unsafe extern "C" fn(cb: MessageReceiveFn, user_data: usize) -> i32>,
    pub register_chat_action: Option<unsafe extern "C" fn(id: *const c_char, label: *const c_char, click_cb: ChatActionClickFn, user_data: usize) -> i32>,
    pub get_object_field: Option<unsafe extern "C" fn(obj: JObject, field_name: *const c_char) -> JObject>,
    pub set_object_field: Option<unsafe extern "C" fn(obj: JObject, field_name: *const c_char, val: JObject)>,
    pub get_int_field: Option<unsafe extern "C" fn(obj: JObject, field_name: *const c_char) -> i32>,
    pub set_int_field: Option<unsafe extern "C" fn(obj: JObject, field_name: *const c_char, val: i32)>,
    pub get_long_field: Option<unsafe extern "C" fn(obj: JObject, field_name: *const c_char) -> i64>,
    pub set_long_field: Option<unsafe extern "C" fn(obj: JObject, field_name: *const c_char, val: i64)>,
    pub get_bool_field: Option<unsafe extern "C" fn(obj: JObject, field_name: *const c_char) -> bool>,
    pub set_bool_field: Option<unsafe extern "C" fn(obj: JObject, field_name: *const c_char, val: bool)>,
    pub get_string_utf: Option<unsafe extern "C" fn(jstr: JObject, out_buf: *mut c_char, max_len: u32) -> i32>,
    pub create_string_utf: Option<unsafe extern "C" fn(str: *const c_char) -> JObject>,
    pub get_attached_field: Option<unsafe extern "C" fn(obj: JObject, key: *const c_char) -> JObject>,
    pub set_attached_field: Option<unsafe extern "C" fn(obj: JObject, key: *const c_char, val: JObject)>,
    pub get_state_value: Option<unsafe extern "C" fn(state_obj: JObject) -> JObject>,
    pub set_state_value: Option<unsafe extern "C" fn(state_obj: JObject, val: JObject)>,
    pub modifier_empty: Option<unsafe extern "C" fn() -> JObject>,
    pub modifier_size: Option<unsafe extern "C" fn(modifier: JObject, width_dp: f32, height_dp: f32) -> JObject>,
    pub modifier_width: Option<unsafe extern "C" fn(modifier: JObject, width_dp: f32) -> JObject>,
    pub modifier_height: Option<unsafe extern "C" fn(modifier: JObject, height_dp: f32) -> JObject>,
    pub modifier_fill_max_width: Option<unsafe extern "C" fn(modifier: JObject) -> JObject>,
    pub modifier_fill_max_height: Option<unsafe extern "C" fn(modifier: JObject) -> JObject>,
    pub modifier_padding: Option<unsafe extern "C" fn(modifier: JObject, start_dp: f32, top_dp: f32, end_dp: f32, bottom_dp: f32) -> JObject>,
    pub modifier_background: Option<unsafe extern "C" fn(modifier: JObject, argb: u32, corner_radius_dp: f32) -> JObject>,
    pub modifier_border: Option<unsafe extern "C" fn(modifier: JObject, width_dp: f32, argb: u32, corner_radius_dp: f32) -> JObject>,
    pub modifier_clickable: Option<unsafe extern "C" fn(modifier: JObject, on_click_lambda: JObject) -> JObject>,
    pub modifier_weight: Option<unsafe extern "C" fn(scope: JObject, modifier: JObject, weight: f32) -> JObject>,
    pub compose_row: Option<unsafe extern "C" fn(composer: JObject, modifier: JObject, content_lambda: JObject)>,
    pub compose_column: Option<unsafe extern "C" fn(composer: JObject, modifier: JObject, content_lambda: JObject)>,
    pub compose_box: Option<unsafe extern "C" fn(composer: JObject, modifier: JObject, content_lambda: JObject)>,
    pub compose_spacer: Option<unsafe extern "C" fn(composer: JObject, modifier: JObject)>,
    pub compose_text: Option<unsafe extern "C" fn(composer: JObject, text: *const c_char, modifier: JObject, argb: u32, font_size_sp: f32, is_bold: bool, max_lines: i32)>,
    pub compose_image: Option<unsafe extern "C" fn(composer: JObject, bitmap: JObject, modifier: JObject)>,
    pub get_avatar_bitmap: Option<unsafe extern "C" fn(path: *const c_char) -> JObject>,
}

static METADATA: PluginMetadata = PluginMetadata {
    name: b"AutoEmoji\0".as_ptr().cast(),
    author: b"@nyanplugins\0".as_ptr().cast(),
    version: b"1.0.0\0".as_ptr().cast(),
    description: b"Replacing all emoji words (like \":banana:\", \":shrug:\") to emoji\0".as_ptr().cast(),
};

const EMOJI: &[(&str, &str)] = &[
    (":banana:", "🍌"),
    (":fire:", "🔥"),
    (":shrug:", "🤷‍♂️"),
    (":heart:", "❤️"),
    (":apple:", "🍎"),
    (":thumbsup:", "👍"),
    (":cat:", "🐱"),
    (":dog:", "🐶"),
    (":monkey:", "🐒"),
    (":coffee:", "☕️"),
    (":rocket:", "🚀"),
    (":ok_hand:", "👌"),
    (":skull:", "💀"),
    (":smile:", "😄")
];

thread_local! {
    static OUT: RefCell<Option<CString>> = const { RefCell::new(None) };
}

#[unsafe(no_mangle)]
pub extern "C" fn mandagram_plugin_metadata() -> *const PluginMetadata { &METADATA }

fn replaceWordsToEmoji(text: &str) -> String {
    let mut withEmoji = text.to_owned();
    for (word, emoji) in EMOJI { withEmoji = withEmoji.replace(word, emoji); }
    withEmoji
}

unsafe extern "C" fn onSendMessageHook(_: i64, text: *const c_char, _: usize) -> *const c_char {
    let res = catch_unwind(AssertUnwindSafe(|| {
        let original = unsafe { CStr::from_ptr(text) }.to_string_lossy();
        let withEmoji = replaceWordsToEmoji(&original);

        if withEmoji == original { return text; }

        match CString::new(withEmoji) {
            Ok(c) => OUT.with(|o| {
                let mut o = o.borrow_mut();
                *o = Some(c);
                o.as_ref().unwrap().as_ptr()
            }),
            Err(_) => text,
        }
    }));
    res.unwrap_or(text)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mandagram_plugin_init(api: *const PluginApi) -> bool {
    let Some(api) = api.as_ref() else { return false; };
    let Some(register) = api.register_message_send_hook else { return false };

    register(Some(onSendMessageHook), None, 0) == 0
}