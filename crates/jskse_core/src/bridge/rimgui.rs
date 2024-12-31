/*
God kill me, please.
This is a bridge file for imgui because imgui-rs isn't as good as I'd hoped.
Maybe it's a pitfall of using SKSE and DirectX11 but who knows. Copilot certainly
makes copying and pasting easier so I can just auto-fill most of it. Perhaps this
would be better as an auto-generated file but I don't have the time for that.
*/

#[cxx::bridge]
mod imgui_rs {
    #[namespace = "rimgui"]
    unsafe extern "C++" {
        include!("PCH.h");
        include!("bridge/rimgui.h");

        pub fn Begin(name: String) -> bool;
        pub fn End();
        pub fn BeginChild(
            name: String,
            size: &Vec<f32>,
            child_flags: i32,
            window_flags: i32,
        ) -> bool;
        pub fn EndChild();
        pub fn IsWindowAppearing() -> bool;
        pub fn IsWindowCollapsed() -> bool;
        pub fn IsWindowFocused(flags: i32) -> bool;
        pub fn IsWindowHovered(flags: i32) -> bool;
        pub fn GetWindowPos() -> Vec<f32>;
        pub fn GetWindowSize() -> Vec<f32>;
        pub fn GetWindowWidth() -> f32;
        pub fn GetWindowHeight() -> f32;
        pub fn SetNextWindowPos(pos: &Vec<f32>, cond: i32, pivot: &Vec<f32>);
        pub fn SetNextWindowSize(size: &Vec<f32>, cond: i32);
        pub fn SetNextWindowSizeConstraints(size_min: &Vec<f32>, size_max: &Vec<f32>);
        pub fn SetNextWindowContentSize(size: &Vec<f32>);
        pub fn SetNextWindowCollapsed(collapsed: bool, cond: i32);
        pub fn SetNextWindowFocus();
        pub fn SetNextWindowBgAlpha(alpha: f32);
        pub fn SetWindowPos(pos: &Vec<f32>, cond: i32);
        pub fn SetWindowSize(size: &Vec<f32>, cond: i32);
        pub fn SetWindowCollapsed(collapsed: bool, cond: i32);
        pub fn SetWindowFocus();
        pub fn SetWindowFontScale(scale: f32);
        pub fn SetWindowPosByName(name: String, pos: &Vec<f32>, cond: i32);
        pub fn SetWindowSizeByName(name: String, size: &Vec<f32>, cond: i32);
        pub fn SetWindowCollapsedByName(name: String, collapsed: bool, cond: i32);
        pub fn SetWindowFocusByName(name: String);

        pub fn GetScrollX() -> f32;
        pub fn GetScrollY() -> f32;
        pub fn SetScrollX(scroll_x: f32);
        pub fn SetScrollY(scroll_y: f32);
        pub fn GetScrollMaxX() -> f32;
        pub fn GetScrollMaxY() -> f32;
        pub fn SetScrollHereX(center_x_ratio: f32);
        pub fn SetScrollHereY(center_y_ratio: f32);
        pub fn SetScrollFromPosX(pos_x: f32, center_x_ratio: f32);
        pub fn SetScrollFromPosY(pos_y: f32, center_y_ratio: f32);
    }
}

pub struct ImVec2 {
    x: f32,
    y: f32,
}

pub fn begin(name: &str) -> bool {
    imgui_rs::Begin(name.to_string())
}

pub fn end() {
    imgui_rs::End()
}

pub fn begin_child(
    name: &str,
    size: ImVec2,
    child_flags: Option<i32>,
    window_flags: Option<i32>,
) -> bool {
    imgui_rs::BeginChild(
        name.to_string(),
        &vec![size.x, size.y],
        child_flags.unwrap_or(0),
        window_flags.unwrap_or(0),
    )
}

pub fn end_child() {
    imgui_rs::EndChild()
}

pub fn is_window_appearing() -> bool {
    imgui_rs::IsWindowAppearing()
}

pub fn is_window_collapsed() -> bool {
    imgui_rs::IsWindowCollapsed()
}

pub fn is_window_focused(flags: i32) -> bool {
    imgui_rs::IsWindowFocused(flags)
}

pub fn is_window_hovered(flags: i32) -> bool {
    imgui_rs::IsWindowHovered(flags)
}

pub fn get_window_pos() -> ImVec2 {
    let pos = imgui_rs::GetWindowPos();
    ImVec2 {
        x: pos[0],
        y: pos[1],
    }
}

pub fn get_window_size() -> ImVec2 {
    let size = imgui_rs::GetWindowSize();
    ImVec2 {
        x: size[0],
        y: size[1],
    }
}

pub fn get_window_width() -> f32 {
    imgui_rs::GetWindowWidth()
}

pub fn get_window_height() -> f32 {
    imgui_rs::GetWindowHeight()
}

pub fn set_next_window_pos(pos: ImVec2, cond: Option<i32>, pivot: Option<ImVec2>) {
    imgui_rs::SetNextWindowPos(
        &vec![pos.x, pos.y],
        cond.unwrap_or(0),
        &pivot.map_or(vec![0.0, 0.0], |p| vec![p.x, p.y]),
    )
}

pub fn set_next_window_size(size: ImVec2, cond: Option<i32>) {
    imgui_rs::SetNextWindowSize(&vec![size.x, size.y], cond.unwrap_or(0))
}

pub fn set_next_window_size_constraints(size_min: ImVec2, size_max: ImVec2) {
    imgui_rs::SetNextWindowSizeConstraints(
        &vec![size_min.x, size_min.y],
        &vec![size_max.x, size_max.y],
    )
}

pub fn set_next_window_content_size(size: ImVec2) {
    imgui_rs::SetNextWindowContentSize(&vec![size.x, size.y])
}

pub fn set_next_window_collapsed(collapsed: bool, cond: Option<i32>) {
    imgui_rs::SetNextWindowCollapsed(collapsed, cond.unwrap_or(0))
}

pub fn set_next_window_focus() {
    imgui_rs::SetNextWindowFocus()
}

pub fn set_next_window_bg_alpha(alpha: f32) {
    imgui_rs::SetNextWindowBgAlpha(alpha)
}

pub fn set_window_pos(pos: ImVec2, cond: Option<i32>) {
    imgui_rs::SetWindowPos(&vec![pos.x, pos.y], cond.unwrap_or(0))
}

pub fn set_window_size(size: ImVec2, cond: Option<i32>) {
    imgui_rs::SetWindowSize(&vec![size.x, size.y], cond.unwrap_or(0))
}

pub fn set_window_collapsed(collapsed: bool, cond: i32) {
    imgui_rs::SetWindowCollapsed(collapsed, cond)
}

pub fn set_window_focus() {
    imgui_rs::SetWindowFocus()
}

pub fn set_window_font_scale(scale: f32) {
    imgui_rs::SetWindowFontScale(scale)
}

pub fn set_window_pos_by_name(name: &str, pos: ImVec2, cond: Option<i32>) {
    imgui_rs::SetWindowPosByName(name.to_string(), &vec![pos.x, pos.y], cond.unwrap_or(0))
}

pub fn set_window_size_by_name(name: &str, size: ImVec2, cond: Option<i32>) {
    imgui_rs::SetWindowSizeByName(name.to_string(), &vec![size.x, size.y], cond.unwrap_or(0))
}

pub fn set_window_collapsed_by_name(name: &str, collapsed: bool, cond: Option<i32>) {
    imgui_rs::SetWindowCollapsedByName(name.to_string(), collapsed, cond.unwrap_or(0))
}

pub fn set_window_focus_by_name(name: &str) {
    imgui_rs::SetWindowFocusByName(name.to_string())
}

pub fn get_scroll_x() -> f32 {
    imgui_rs::GetScrollX()
}

pub fn get_scroll_y() -> f32 {
    imgui_rs::GetScrollY()
}

pub fn set_scroll_x(scroll_x: f32) {
    imgui_rs::SetScrollX(scroll_x)
}

pub fn set_scroll_y(scroll_y: f32) {
    imgui_rs::SetScrollY(scroll_y)
}

pub fn get_scroll_max_x() -> f32 {
    imgui_rs::GetScrollMaxX()
}

pub fn get_scroll_max_y() -> f32 {
    imgui_rs::GetScrollMaxY()
}

pub fn set_scroll_here_x(center_x_ratio: Option<f32>) {
    imgui_rs::SetScrollHereX(center_x_ratio.unwrap_or(0.5))
}

pub fn set_scroll_here_y(center_y_ratio: Option<f32>) {
    imgui_rs::SetScrollHereY(center_y_ratio.unwrap_or(0.5))
}

pub fn set_scroll_from_pos_x(pos_x: f32, center_x_ratio: Option<f32>) {
    imgui_rs::SetScrollFromPosX(pos_x, center_x_ratio.unwrap_or(0.5))
}

pub fn set_scroll_from_pos_y(pos_y: f32, center_y_ratio: Option<f32>) {
    imgui_rs::SetScrollFromPosY(pos_y, center_y_ratio.unwrap_or(0.5))
}
