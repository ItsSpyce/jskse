/*
God kill me, please.
This is a bridge file for imgui because imgui-rs isn't as good as I'd hoped.
Maybe it's a pitfall of using SKSE and DirectX11 but who knows. Copilot certainly
makes copying and pasting easier so I can just auto-fill most of it. Perhaps this
would be better as an auto-generated file but I don't have the time for that.
*/

use cxx::{type_id, CxxVector, ExternType};
use imgui_rs::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ImGuiTableSortSpecs {
    pub specs: *mut ImGuiTableColumnSortSpecs,
    pub specs_count: i32,
    pub specs_dirty: bool,
}

unsafe impl ExternType for ImGuiTableSortSpecs {
    type Id = type_id!("TableSortSpecs");
    type Kind = cxx::kind::Opaque;
}

pub struct ImGuiTableColumnSortSpecs {
    pub column_idx: i32,
    pub sort_order: i32,
}

#[cxx::bridge]
pub mod imgui_rs {

    #[namespace = "rimgui"]
    unsafe extern "C++" {
        include!("PCH.h");
        include!("bridge/rimgui.h");

        type ImGuiTableSortSpecs;

        pub unsafe fn Begin(name: &CxxString, open: *mut bool, flags: i32) -> bool;
        pub fn End();
        pub fn BeginChild(
            name: &CxxString,
            size: &[f32; 2],
            child_flags: i32,
            window_flags: i32,
        ) -> bool;
        pub fn EndChild();
        pub fn IsWindowAppearing() -> bool;
        pub fn IsWindowCollapsed() -> bool;
        pub fn IsWindowFocused(flags: i32) -> bool;
        pub fn IsWindowHovered(flags: i32) -> bool;
        pub fn GetWindowPos() -> [f32; 2];
        pub fn GetWindowSize() -> [f32; 2];
        pub fn GetWindowWidth() -> f32;
        pub fn GetWindowHeight() -> f32;
        pub fn SetNextWindowPos(pos: &[f32; 2], cond: i32, pivot: &[f32; 2]);
        pub fn SetNextWindowSize(size: &[f32; 2], cond: i32);
        pub fn SetNextWindowSizeConstraints(size_min: &[f32; 2], size_max: &[f32; 2]);
        pub fn SetNextWindowContentSize(size: &[f32; 2]);
        pub fn SetNextWindowCollapsed(collapsed: bool, cond: i32);
        pub fn SetNextWindowFocus();
        pub fn SetNextWindowBgAlpha(alpha: f32);
        pub fn SetWindowPos(pos: &[f32; 2], cond: i32);
        pub fn SetWindowSize(size: &[f32; 2], cond: i32);
        pub fn SetWindowCollapsed(collapsed: bool, cond: i32);
        pub fn SetWindowFocus();
        pub fn SetWindowFontScale(scale: f32);
        pub fn SetWindowPosByName(name: &CxxString, pos: &[f32; 2], cond: i32);
        pub fn SetWindowSizeByName(name: &CxxString, size: &[f32; 2], cond: i32);
        pub fn SetWindowCollapsedByName(name: &CxxString, collapsed: bool, cond: i32);
        pub fn SetWindowFocusByName(name: &CxxString);

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

        pub fn PopFont();
        pub fn PushStyleColor(idx: i32, col: &[f32; 4]);
        pub fn PopStyleColor(count: i32);
        pub fn PushStyleVar(idx: i32, val: &[f32; 2]);
        pub fn PopStyleVar(count: i32);
        pub fn PushItemFlag(idx: i32, val: bool);
        pub fn PopItemFlag();
        pub fn PushItemWidth(item_width: f32);
        pub fn PopItemWidth();
        pub fn SetNextItemWidth(item_width: f32);
        pub fn CalcItemWidth() -> f32;
        pub fn PushTextWrapPos(wrap_local_pos_x: f32);
        pub fn PopTextWrapPos();
        pub fn GetFontSize() -> f32;
        pub fn GetFontTextUvWhitePixel() -> [f32; 2];
        pub fn GetColorU32(color: &[f32; 4]) -> u32;
        pub fn GetColorU32FromStyle(idx: i32, alpha_mul: f32) -> u32;
        pub fn GetStyleColorVec4(idx: i32) -> [f32; 4];
        pub fn GetCursorScreenPos() -> [f32; 2];
        pub fn SetCursorScreenPos(pos: &[f32; 2]);
        pub fn GetContentRegionAvail() -> [f32; 2];
        pub fn GetCursorPos() -> [f32; 2];
        pub fn GetCursorPosX() -> f32;
        pub fn GetCursorPosY() -> f32;
        pub fn SetCursorPos(pos: &[f32; 2]);
        pub fn SetCursorPosX(x: f32);
        pub fn SetCursorPosY(y: f32);
        pub fn GetCursorStartPos() -> [f32; 2];
        pub fn Separator();
        pub fn SameLine(offset_from_start_x: f32, spacing: f32);
        pub fn NewLine();
        pub fn Spacing();
        pub fn Dummy(size: &[f32; 2]);
        pub fn Indent(indent_w: f32);
        pub fn Unindent(indent_w: f32);
        pub fn BeginGroup();
        pub fn EndGroup();
        pub fn AlignTextToFramePadding();
        pub fn GetTextLineHeight() -> f32;
        pub fn GetTextLineHeightWithSpacing() -> f32;
        pub fn GetFrameHeight() -> f32;
        pub fn GetFrameHeightWithSpacing() -> f32;
        pub fn PushIDString(id: &CxxString);
        pub fn PushID(id: i32);
        pub fn PopID();
        pub fn GetIDFromString(str_id: &CxxString) -> u32;
        pub fn GetID(id: i32) -> u32;
        pub fn TextUnformatted(text: &CxxString);
        pub fn Text(text: &CxxString, args: &CxxVector<CxxString>);
        pub fn TextColored(col: &[f32; 4], text: &CxxString, args: &CxxVector<CxxString>);
        pub fn TextDisabled(text: &CxxString, args: &CxxVector<CxxString>);
        pub fn TextWrapped(text: &CxxString, args: &CxxVector<CxxString>);
        pub fn LabelText(label: &CxxString, text: &CxxString, args: &CxxVector<CxxString>);
        pub fn BulletText(text: &CxxString, args: &CxxVector<CxxString>);
        pub fn SeparatorText(text: &CxxString);
        pub fn Button(label: &CxxString, size: &[f32; 2]) -> bool;
        pub fn SmallButton(label: &CxxString) -> bool;
        pub fn InvisibleButton(str_id: &CxxString, size: &[f32; 2], flags: i32) -> bool;
        pub fn ArrowButton(str_id: &CxxString, dir: i32) -> bool;
        pub unsafe fn Checkbox(label: &CxxString, v: *mut bool) -> bool;
        pub unsafe fn CheckboxFlags(label: &CxxString, flags: *mut i32, flags_value: i32) -> bool;
        pub fn RadioButtonBool(label: &CxxString, active: bool) -> bool;
        pub unsafe fn RadioButton(label: &CxxString, v: *mut i32, v_button: i32) -> bool;
        pub fn ProgressBar(fraction: f32, size_arg: &[f32; 2], overlay: &CxxString);
        pub fn Bullet();
        pub fn Image(
            user_texture_bytes: &Vec<u8>,
            size: &[f32; 2],
            uv0: &[f32; 2],
            uv1: &[f32; 2],
            tint_col: &[f32; 4],
            border_col: &[f32; 4],
        );
        pub fn ImageFromFile(
            filename: &CxxString,
            size: &[f32; 2],
            uv0: &[f32; 2],
            uv1: &[f32; 2],
            tint_col: &[f32; 4],
            border_col: &[f32; 4],
        );
        pub fn ImageButton(
            str_id: &CxxString,
            user_texture_bytes: &Vec<u8>,
            size: &[f32; 2],
            uv0: &[f32; 2],
            uv1: &[f32; 2],
            bg_col: &[f32; 4],
            tint_col: &[f32; 4],
        ) -> bool;
        pub fn ImageButtonFromFile(
            str_id: &CxxString,
            filename: &CxxString,
            size: &[f32; 2],
            uv0: &[f32; 2],
            uv1: &[f32; 2],
            bg_col: &[f32; 4],
            tint_col: &[f32; 4],
        ) -> bool;

        pub fn BeginCombo(label: &CxxString, preview_value: &CxxString, flags: i32) -> bool;
        pub fn EndCombo();
        pub unsafe fn Combo(
            label: &CxxString,
            current_item: *mut i32,
            items: &Vec<String>,
            height_in_items: i32,
        ) -> bool;
        pub unsafe fn DragFloat(
            label: &CxxString,
            v: *mut f32,
            v_speed: f32,
            v_min: f32,
            v_max: f32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn DragFloat2(
            label: &CxxString,
            v: *mut [f32; 2],
            v_speed: f32,
            v_min: f32,
            v_max: f32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn DragFloat3(
            label: &CxxString,
            v: *mut [f32; 3],
            v_speed: f32,
            v_min: f32,
            v_max: f32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn DragFloat4(
            label: &CxxString,
            v: *mut [f32; 4],
            v_speed: f32,
            v_min: f32,
            v_max: f32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn DragFloatRange2(
            label: &CxxString,
            v_current_min: *mut f32,
            v_current_max: *mut f32,
            v_speed: f32,
            v_min: f32,
            v_max: f32,
            format: &CxxString,
            format_max: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn DragInt(
            label: &CxxString,
            v: *mut i32,
            v_speed: f32,
            v_min: i32,
            v_max: i32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn DragInt2(
            label: &CxxString,
            v: *mut [i32; 2],
            v_speed: f32,
            v_min: i32,
            v_max: i32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn DragInt3(
            label: &CxxString,
            v: *mut [i32; 3],
            v_speed: f32,
            v_min: i32,
            v_max: i32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn DragInt4(
            label: &CxxString,
            v: *mut [i32; 4],
            v_speed: f32,
            v_min: i32,
            v_max: i32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn DragIntRange2(
            label: &CxxString,
            v_current_min: *mut i32,
            v_current_max: *mut i32,
            v_speed: f32,
            v_min: i32,
            v_max: i32,
            format: &CxxString,
            format_max: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn SliderFloat(
            label: &CxxString,
            v: *mut f32,
            v_min: f32,
            v_max: f32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn SliderFloat2(
            label: &CxxString,
            v: *mut [f32; 2],
            v_min: f32,
            v_max: f32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn SliderFloat3(
            label: &CxxString,
            v: *mut [f32; 3],
            v_min: f32,
            v_max: f32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn SliderFloat4(
            label: &CxxString,
            v: *mut [f32; 4],
            v_min: f32,
            v_max: f32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn SliderAngle(
            label: &CxxString,
            v_rad: *mut f32,
            v_degrees_min: f32,
            v_degrees_max: f32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn SliderInt(
            label: &CxxString,
            v: *mut i32,
            v_min: i32,
            v_max: i32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn SliderInt2(
            label: &CxxString,
            v: *mut [i32; 2],
            v_min: i32,
            v_max: i32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn SliderInt3(
            label: &CxxString,
            v: *mut [i32; 3],
            v_min: i32,
            v_max: i32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn SliderInt4(
            label: &CxxString,
            v: *mut [i32; 4],
            v_min: i32,
            v_max: i32,
            format: &CxxString,
            flags: i32,
        ) -> bool;

        pub fn InputText(label: &CxxString, buf: &mut Vec<u8>, flags: i32) -> bool;
        pub fn InputTextWithHint(
            label: &CxxString,
            hint: &CxxString,
            buf: &mut Vec<u8>,
            flags: i32,
        ) -> bool;
        pub unsafe fn InputFloat(
            label: &CxxString,
            v: *mut f32,
            step: f32,
            step_fast: f32,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn InputFloat2(
            label: &CxxString,
            v: *mut [f32; 2],
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn InputFloat3(
            label: &CxxString,
            v: *mut [f32; 3],
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn InputFloat4(
            label: &CxxString,
            v: *mut [f32; 4],
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn InputInt(
            label: &CxxString,
            v: *mut i32,
            step: i32,
            step_fast: i32,
            flags: i32,
        ) -> bool;
        pub unsafe fn InputInt2(label: &CxxString, v: *mut [i32; 2], flags: i32) -> bool;
        pub unsafe fn InputInt3(label: &CxxString, v: *mut [i32; 3], flags: i32) -> bool;
        pub unsafe fn InputInt4(label: &CxxString, v: *mut [i32; 4], flags: i32) -> bool;
        pub unsafe fn InputDouble(
            label: &CxxString,
            v: *mut f64,
            step: f64,
            step_fast: f64,
            format: &CxxString,
            flags: i32,
        ) -> bool;
        pub unsafe fn ColorEdit3(label: &CxxString, col: *mut [f32; 3], flags: i32) -> bool;
        pub unsafe fn ColorEdit4(label: &CxxString, col: *mut [f32; 4], flags: i32) -> bool;
        pub unsafe fn ColorPicker3(label: &CxxString, col: *mut [f32; 3], flags: i32) -> bool;
        pub unsafe fn ColorPicker4(
            label: &CxxString,
            col: *mut [f32; 4],
            flags: i32,
            ref_col: &[f32; 4],
        ) -> bool;
        pub unsafe fn ColorButton(
            desc_id: &CxxString,
            col: *mut [f32; 4],
            flags: i32,
            size: &[f32; 2],
        ) -> bool;
        pub fn SetColorEditOptions(flags: i32);

        pub fn TreeNode(label: &CxxString) -> bool;
        pub fn TreeNodeWithId(str_id: &CxxString, fmt: &CxxString) -> bool;
        pub fn TreeNodeEx(label: &CxxString, flags: i32) -> bool;
        pub fn TreeNodeExWithId(str_id: &CxxString, flags: i32, fmt: &CxxString) -> bool;
        pub fn TreePush(label: &CxxString);
        pub fn TreePop();
        pub fn GetTreeNodeToLabelSpacing() -> f32;
        pub fn CollapsingHeader(label: &CxxString, flags: i32) -> bool;
        pub unsafe fn CollapsingHeaderCtrl(label: &CxxString, open: *mut bool, flags: i32) -> bool;
        pub fn SetNextItemOpen(is_open: bool, cond: i32);
        pub fn SetNextItemStorageID(storage_id: u32);
        pub unsafe fn Selectable(
            label: &CxxString,
            selected: *mut bool,
            flags: i32,
            size: &[f32; 2],
        ) -> bool;
        pub fn BeginListBox(label: &CxxString, size: &[f32; 2]) -> bool;
        pub fn EndListBox();
        pub unsafe fn ListBox(
            label: &CxxString,
            current_item: *mut i32,
            items: &Vec<String>,
            height_in_items: i32,
        ) -> bool;
        pub fn BeginMainMenuBar() -> bool;
        pub fn EndMainMenuBar();
        pub fn BeginMenuBar() -> bool;
        pub fn EndMenuBar();
        pub fn BeginMenu(label: &CxxString, enabled: bool) -> bool;
        pub fn EndMenu();
        pub fn MenuItem(
            label: &CxxString,
            shortcut: &CxxString,
            selected: bool,
            enabled: bool,
        ) -> bool;

        pub fn BeginTooltip();
        pub fn EndTooltip();
        pub fn SetTooltip(text: &CxxString);

        pub fn BeginItemTooltip() -> bool;
        pub fn SetItemTooltip(text: &CxxString);

        pub fn BeginPopup(str_id: &CxxString, flags: i32) -> bool;
        pub unsafe fn BeginPopupModal(name: &CxxString, open: *mut bool, flags: i32) -> bool;
        pub fn EndPopup();

        pub fn OpenPopup(str_id: &CxxString, flags: i32);
        pub fn OpenPopupOnItemClick(str_id: &CxxString, popup_flags: i32);
        pub fn CloseCurrentPopup();

        pub fn BeginPopupContextItem(str_id: &CxxString, mouse_button: i32) -> bool;
        pub fn BeginPopupContextWindow(str_id: &CxxString, mouse_button: i32) -> bool;
        pub fn BeginPopupContextVoid(str_id: &CxxString, mouse_button: i32) -> bool;

        pub fn IsPopupOpen(str_id: &CxxString, flags: i32) -> bool;

        pub fn BeginTable(
            str_id: &CxxString,
            columns_count: i32,
            flags: i32,
            outer_size: &[f32; 2],
            inner_width: f32,
        ) -> bool;
        pub fn EndTable();
        pub fn TableNextRow(row_flags: i32, min_row_height: f32);
        pub fn TableNextColumn() -> bool;
        pub fn TableSetColumnIndex(column_n: i32) -> bool;
        pub fn TableSetupColumn(
            label: &CxxString,
            flags: i32,
            init_width_or_weight: f32,
            user_id: u32,
        );
        pub fn TableSetupScrollFreeze(cols: i32, rows: i32);
        pub fn TableHeadersRow();
        pub fn TableHeader(label: &CxxString);
        pub fn TableGetSortSpecs() -> *mut ImGuiTableSortSpecs;
        pub fn TableGetColumnCount() -> i32;
        pub fn TableGetColumnIndex() -> i32;
        pub fn TableGetRowIndex() -> i32;
        pub fn TableGetColumnName(column_n: i32) -> String;
        pub fn TableGetColumnFlags(column_n: i32) -> i32;
        pub fn TableSetColumnEnabled(column_n: i32, enabled: bool);
        pub fn TableSetBgColor(target: i32, color: u32, column_n: i32);

        pub fn Columns(count: i32, id: &CxxString, border: bool);
        pub fn NextColumn();
        pub fn GetColumnIndex() -> i32;
        pub fn GetColumnWidth(column_index: i32) -> f32;
        pub fn SetColumnWidth(column_index: i32, width: f32);
        pub fn GetColumnOffset(column_index: i32) -> f32;
        pub fn SetColumnOffset(column_index: i32, offset_x: f32);
        pub fn GetColumnsCount() -> i32;

        pub fn BeginTabBar(str_id: &CxxString, flags: i32) -> bool;
        pub fn EndTabBar();
        pub unsafe fn BeginTabItem(str_id: &CxxString, open: *mut bool, flags: i32) -> bool;
        pub fn EndTabItem();
        pub fn TabItemButton(label: &CxxString, flags: i32) -> bool;
        pub fn SetTabItemClosed(tab_or_docked_window_label: &CxxString);

        pub fn BeginDisabled(disabled: bool);
        pub fn EndDisabled();

        pub fn PushClipRect(
            clip_rect_min: &[f32; 2],
            clip_rect_max: &[f32; 2],
            intersect_with_current_clip_rect: bool,
        );
        pub fn PopClipRect();

        pub fn SetItemDefaultFocus();
        pub fn SetKeyboardFocusHere(offset: i32);

        pub fn IsItemHovered(flags: i32) -> bool;
        pub fn IsItemActive() -> bool;
        pub fn IsItemFocused() -> bool;
        pub fn IsItemClicked(mouse_button: i32) -> bool;
        pub fn IsItemVisible() -> bool;
        pub fn IsItemEdited() -> bool;
        pub fn IsItemActivated() -> bool;
        pub fn IsItemDeactivated() -> bool;
        pub fn IsItemDeactivatedAfterEdit() -> bool;
        pub fn IsItemToggledOpen() -> bool;
        pub fn IsAnyItemHovered() -> bool;
        pub fn GetItemRectMin() -> [f32; 2];
        pub fn GetItemRectMax() -> [f32; 2];
        pub fn GetItemRectSize() -> [f32; 2];
        pub fn SetItemAllowOverlap();

        pub fn IsRectVisible(rect_min_or_size: &[f32; 2], rect_max_or_none: &[f32; 2]) -> bool;
        pub fn GetTime() -> f64;
        pub fn GetFrameCount() -> i32;
        pub fn GetStyleColorName(idx: i32) -> String;
        pub fn BeginChildFrame(id: u32, size: &[f32; 2], flags: i32) -> bool;
        pub fn EndChildFrame();

        pub fn CalcTextSize(
            text: &CxxString,
            text_end: &CxxString,
            hide_text_after_double_hash: bool,
            wrap_width: f32,
        ) -> [f32; 2];
        pub fn ColorConvertU32ToFloat4(color: u32) -> [f32; 4];
        pub fn ColorConvertFloat4ToU32(col: &[f32; 4]) -> u32;
        pub fn ColorConvertRGBToHSV(rgb: &[f32; 4]) -> [f32; 4];
        pub fn ColorConvertHSVToRGB(hsv: &[f32; 4]) -> [f32; 4];

        pub fn IsKeyDown(user_key_index: i32) -> bool;
        pub fn IsKeyPressed(user_key_index: i32, repeat: bool) -> bool;
        pub fn IsKeyReleased(user_key_index: i32) -> bool;
        pub fn GetKeyPressedAmount(key_index: i32, repeat_delay: f32, rate: f32) -> i32;
        pub fn GetKeyName(user_key_index: i32) -> String;
        pub fn SetNextFrameWantCaptureKeyboard(value: i32);

        pub fn IsMouseDown(button: i32) -> bool;
        pub fn IsMouseClicked(button: i32, repeat: bool) -> bool;
        pub fn IsMouseReleased(button: i32) -> bool;
        pub fn IsMouseDoubleClicked(button: i32) -> bool;
        pub fn GetMouseClickedCount(button: i32) -> i32;
        pub fn IsMouseHoveringRect(r_min: &[f32; 2], r_max: &[f32; 2], clip: bool) -> bool;
        pub fn IsMousePosValid(mouse_pos: &[f32; 2]) -> bool;
        pub fn IsAnyMouseDown() -> bool;
        pub fn GetMousePos() -> [f32; 2];
        pub fn GetMousePosOnOpeningCurrentPopup() -> [f32; 2];
        pub fn IsMouseDragging(button: i32, lock_threshold: f32) -> bool;
        pub fn GetMouseDragDelta(button: i32, lock_threshold: f32) -> [f32; 2];
        pub fn ResetMouseDragDelta(button: i32);
        pub fn GetMouseCursor() -> i32;
        pub fn SetMouseCursor(cursor_type: i32);
        pub fn SetNextFrameWantCaptureMouse(want_capture_mouse: bool);

        pub fn GetClipboardText() -> String;
        pub fn SetClipboardText(text: &CxxString);
    }
}

#[derive(Clone, Copy, Default)]
pub struct ImVec2 {
    x: f32,
    y: f32,
}

impl ImVec2 {
    pub fn new(vec: [f32; 2]) -> ImVec2 {
        Self {
            x: vec[0],
            y: vec[1],
        }
    }
    pub fn to_vec(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

#[derive(Clone, Copy, Default)]
pub struct ImVec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl ImVec4 {
    pub fn new(vec: [f32; 4]) -> ImVec4 {
        Self {
            x: vec[0],
            y: vec[1],
            z: vec[2],
            w: vec[3],
        }
    }
    pub fn to_vec(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

#[derive(Clone, Copy, Default)]
pub struct ImRgb {
    r: f32,
    g: f32,
    b: f32,
}

impl ImRgb {
    pub fn new(vec: [f32; 3]) -> ImRgb {
        Self {
            r: vec[0],
            g: vec[1],
            b: vec[2],
        }
    }
    pub fn to_vec(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

#[derive(Clone, Copy, Default)]
pub struct ImRgba {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl ImRgba {
    pub fn new(vec: [f32; 4]) -> ImRgba {
        Self {
            r: vec[0],
            g: vec[1],
            b: vec[2],
            a: vec[3],
        }
    }
    pub fn to_vec(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

#[derive(Clone, Copy, Default)]
pub struct ImHsva {
    h: f32,
    s: f32,
    v: f32,
    a: f32,
}

impl ImHsva {
    pub fn new(vec: [f32; 4]) -> ImHsva {
        Self {
            h: vec[0],
            s: vec[1],
            v: vec[2],
            a: vec[3],
        }
    }
    pub fn to_vec(&self) -> [f32; 4] {
        [self.h, self.s, self.v, self.a]
    }
}

pub enum ImGuiWindowFlags {
    None = 0,
    NoTitleBar = 1 << 0,
    NoResize = 1 << 1,
    NoMove = 1 << 2,
    NoScrollbar = 1 << 3,
    NoScrollWithMouse = 1 << 4,
    NoCollapse = 1 << 5,
    AlwaysAutoResize = 1 << 6,
    NoBackground = 1 << 7,
    NoSavedSettings = 1 << 8,
    NoMouseInputs = 1 << 9,
    MenuBar = 1 << 10,
    HorizontalScrollbar = 1 << 11,
    NoFocusOnAppearing = 1 << 12,
    NoBringToFrontOnFocus = 1 << 13,
    AlwaysVerticalScrollbar = 1 << 14,
    AlwaysHorizontalScrollbar = 1 << 15,
    AlwaysUseWindowPadding = 1 << 16,
    NoNavInputs = 1 << 18,
    NoNavFocus = 1 << 19,
    UnsavedDocument = 1 << 20,
    NoNav = (1 << 18) | (1 << 19),
    NoDecoration = (1 << 0) | (1 << 1) | (1 << 3) | (1 << 5),
    NoInputs = (1 << 9) | (1 << 18) | (1 << 19),
    NavFlattened = 1 << 23,
    ChildWindow = 1 << 24,
    Tooltip = 1 << 25,
    Popup = 1 << 26,
    Modal = 1 << 27,
    ChildMenu = 1 << 28,
}

pub enum ImGuiInputTextFlags {
    None = 0,
    CharsDecimal = 1 << 0,
    CharsHexadecimal = 1 << 1,
    CharsUppercase = 1 << 2,
    CharsNoBlank = 1 << 3,
    AutoSelectAll = 1 << 4,
    EnterReturnsTrue = 1 << 5,
    CallbackCompletion = 1 << 6,
    CallbackHistory = 1 << 7,
    CallbackAlways = 1 << 8,
    CallbackCharFilter = 1 << 9,
    AllowTabInput = 1 << 10,
    CtrlEnterForNewLine = 1 << 11,
    NoHorizontalScroll = 1 << 12,
    AlwaysOverwrite = 1 << 13,
    ReadOnly = 1 << 14,
    Password = 1 << 15,
    NoUndoRedo = 1 << 16,
    CharsScientific = 1 << 17,
    CallbackResize = 1 << 18,
    CallbackEdit = 1 << 19,
    EscapeClearsAll = 1 << 20,
}

pub enum ImGuiTreeNodeFlags {
    None = 0,
    Selected = 1 << 0,
    Framed = 1 << 1,
    AllowItemOverlap = 1 << 2,
    NoTreePushOnOpen = 1 << 3,
    NoAutoOpenOnLog = 1 << 4,
    DefaultOpen = 1 << 5,
    OpenOnDoubleClick = 1 << 6,
    OpenOnArrow = 1 << 7,
    Leaf = 1 << 8,
    Bullet = 1 << 9,
    FramePadding = 1 << 10,
    SpanAvailWidth = 1 << 11,
    SpanFullWidth = 1 << 12,
    NavLeftJumpsBackHere = 1 << 13,
    CollapsingHeader = (1 << 1) | (1 << 3) | (1 << 4),
}

pub enum ImGuiPopupFlags {
    NoneOrMouseButtonLeft = 0,
    MouseButtonRight = 1,
    MouseButtonMiddle = 2,
    MouseButtonMask_ = 0x1F,
    NoOpenOverExistingPopup = 1 << 5,
    NoOpenOverItems = 1 << 6,
    AnyPopupId = 1 << 7,
    AnyPopupLevel = 1 << 8,
    AnyPopup = (1 << 7) | (1 << 8),
}

pub enum ImGuiSelectableFlags {
    None = 0,
    DontClosePopups = 1 << 0,
    SpanAllColumns = 1 << 1,
    AllowDoubleClick = 1 << 2,
    Disabled = 1 << 3,
    AllowItemOverlap = 1 << 4,
}

pub enum ImGuiComboFlags {
    None = 0,
    PopupAlignLeft = 1 << 0,
    HeightSmall = 1 << 1,
    HeightRegular = 1 << 2,
    HeightLarge = 1 << 3,
    HeightLargest = 1 << 4,
    NoArrowButton = 1 << 5,
    NoPreview = 1 << 6,
}

pub enum ImGuiTabBarFlags {
    None = 0,
    Reorderable = 1 << 0,
    AutoSelectNewTabs = 1 << 1,
    TabListPopupButton = 1 << 2,
    NoCloseWithMiddleMouseButton = 1 << 3,
    NoTabListScrollingButtons = 1 << 4,
    NoTooltip = 1 << 5,
    FittingPolicyResizeDown = 1 << 6,
    FittingPolicyScroll = 1 << 7,
}

pub enum ImGuiTabItemFlags {
    None = 0,
    UnsavedDocument = 1 << 0,
    SetSelected = 1 << 1,
    NoCloseWithMiddleMouseButton = 1 << 2,
    NoPushId = 1 << 3,
    NoTooltip = 1 << 4,
    NoReorder = 1 << 5,
    Leading = 1 << 6,
    Trailing = 1 << 7,
}

pub enum ImGuiTableFlags {
    None = 0,
    Resizable = 1 << 0,
    Reorderable = 1 << 1,
    Hideable = 1 << 2,
    Sortable = 1 << 3,
    NoSavedSettings = 1 << 4,
    ContextMenuInBody = 1 << 5,
    RowBg = 1 << 6,
    BordersInnerH = 1 << 7,
    BordersOuterH = 1 << 8,
    BordersInnerV = 1 << 9,
    BordersOuterV = 1 << 10,
    BordersH = (1 << 7) | (1 << 8),
    BordersV = (1 << 9) | (1 << 10),
    BordersInner = (1 << 7) | (1 << 9),
    BordersOuter = (1 << 8) | (1 << 10),
    Borders = (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10),
    NoBordersInBody = 1 << 11,
    NoBordersInBodyUntilResize = 1 << 12,
    SizingFixedFit = 1 << 13,
    SizingFixedSame = 2 << 13,
    SizingStretchProp = 3 << 13,
    SizingStretchSame = 4 << 13,
    NoHostExtendX = 1 << 16,
    NoHostExtendY = 1 << 17,
    NoKeepColumnsVisible = 1 << 18,
    PreciseWidths = 1 << 19,
    NoClip = 1 << 20,
    PadOuterX = 1 << 21,
    NoPadOuterX = 1 << 22,
    NoPadInnerX = 1 << 23,
    ScrollX = 1 << 24,
    ScrollY = 1 << 25,
    SortMulti = 1 << 26,
    SortTristate = 1 << 27,
}

pub enum ImGuiTableColumnFlags {
    None = 0,
    Disabled = 1 << 0,
    DefaultHide = 1 << 1,
    DefaultSort = 1 << 2,
    WidthStretch = 1 << 3,
    WidthFixed = 1 << 4,
    NoResize = 1 << 5,
    NoReorder = 1 << 6,
    NoHide = 1 << 7,
    NoClip = 1 << 8,
    NoSort = 1 << 9,
    NoSortAscending = 1 << 10,
    NoSortDescending = 1 << 11,
    NoHeaderLabel = 1 << 12,
    NoHeaderWidth = 1 << 13,
    PreferSortAscending = 1 << 14,
    PreferSortDescending = 1 << 15,
    IndentEnable = 1 << 16,
    IndentDisable = 1 << 17,
    IsEnabled = 1 << 20,
    IsVisible = 1 << 21,
    IsSorted = 1 << 22,
    IsHovered = 1 << 23,
}

pub enum ImGuiTableRowFlags {
    None = 0,
    Headers = 1 << 0,
}

pub enum ImGuiTableBgTarget {
    None = 0,
    RowBg0 = 1,
    RowBg1 = 2,
    CellBg = 3,
}

pub enum ImGuiFocusedFlags {
    None = 0,
    ChildWindows = 1 << 0,
    RootWindow = 1 << 1,
    AnyWindow = 1 << 2,
    NoPopupHierarchy = 1 << 3,
    RootAndChildWindows = (1 << 0) | (1 << 1),
}

pub enum ImGuiHoveredFlags {
    None = 0,
    ChildWindows = 1 << 0,
    RootWindow = 1 << 1,
    AnyWindow = 1 << 2,
    NoPopupHierarchy = 1 << 3,
    AllowWhenBlockedByPopup = 1 << 5,
    AllowWhenBlockedByActiveItem = 1 << 7,
    AllowWhenOverlapped = 1 << 8,
    AllowWhenDisabled = 1 << 9,
    NoNavOverride = 1 << 10,
}

pub enum ImGuiDragDropFlags {
    None = 0,
    SourceNoPreviewTooltip = 1 << 0,
    SourceNoDisableHover = 1 << 1,
    SourceNoHoldToOpenOthers = 1 << 2,
    SourceAllowNullID = 1 << 3,
    SourceExtern = 1 << 4,
    SourceAutoExpirePayload = 1 << 5,
    AcceptBeforeDelivery = 1 << 10,
    AcceptNoDrawDefaultRect = 1 << 11,
    AcceptNoPreviewTooltip = 1 << 12,
    AcceptPeekOnly = (1 << 10) | (1 << 11),
}

pub enum ImGuiDataType {
    S8 = 0,
    U8 = 1,
    S16 = 2,
    U16 = 3,
    S32 = 4,
    U32 = 5,
    S64 = 6,
    U64 = 7,
    Float = 8,
    Double = 9,
    COUNT,
}

pub enum ImGuiDir {
    None = -1,
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
    COUNT,
}

pub enum ImGuiSortDirection {
    None = 0,
    Ascending = 1,
    Descending = 2,
}

pub enum ImGuiKey {
    None = 0,
    Tab = 512,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    PageUp,
    PageDown,
    Home,
    End,
    Insert,
    Delete,
    Backspace,
    Space,
    Enter,
    Escape,
    LeftCtrl,
    LeftShift,
    LeftAlt,
    LeftSuper,
    RightCtrl,
    RightShift,
    RightAlt,
    RightSuper,
    Menu,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Semicolon,
    Equal,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    KeypadZero,
    KeypadOne,
    KeypadTwo,
    KeypadThree,
    KeypadFour,
    KeypadFive,
    KeypadSix,
    KeypadSeven,
    KeypadEight,
    KeypadNine,
    KeypadDecimal,
    KeypadMultiply,
    KeypadSubtract,
    KeypadAdd,
    KeypadEnter,
    KeypadEqual,
    // Gamepad values
    GamepadStart,
    GamepadBack,
    GamepadFaceLeft,
    GamepadFaceRight,
    GamepadFaceUp,
    GamepadFaceDown,
    GamepadDpadLeft,
    GamepadDpadRight,
    GamepadDpadUp,
    GamepadDpadDown,
    GamepadL1,
    GamepadR1,
    GamepadL2,
    GamepadR2,
    GamepadL3,
    GamepadR3,
    GamepadLStickLeft,
    GamepadLStickRight,
    GamepadLStickUp,
    GamepadLStickDown,
    GamepadRStickLeft,
    GamepadRStickRight,
    GamepadRStickUp,
    GamepadRStickDown,
    // Mouse buttons
    MouseLeft,
    MouseRight,
    MouseMiddle,
    MouseX1,
    MouseX2,
    MouseWheelX,
    MouseWheelY,
    // internal reserved
    ReservedForModCtrl,
    ReservedForModShift,
    ReservedForModAlt,
    ReservedForModSuper,
    COUNT,
}

pub enum ImGuiKeyMod {
    None = 0,
    Ctrl = 1 << 12,
    Shift = 1 << 13,
    Alt = 1 << 14,
    Super = 1 << 15,
}

pub enum ImGuiNavInput {
    Activate,
    Cancel,
    Input,
    Menu,
    DpadLeft,
    DpadRight,
    DpadUp,
    DpadDown,
    LStickLeft,
    LStickRight,
    LStickUp,
    LStickDown,
    FocusPrev,
    FocusNext,
    TweakSlow,
    TweakFast,
    COUNT,
}

pub enum ImGuiConfigFlags {
    None = 0,
    NavEnableKeyboard = 1 << 0,
    NavEnableGamepad = 1 << 1,
    NavEnableSetMousePos = 1 << 2,
    NavNoCaptureKeyboard = 1 << 3,
    NoMouse = 1 << 4,
    NoMouseCursorChange = 1 << 5,
    IsSRGB = 1 << 20,
    IsTouchScreen = 1 << 21,
}

pub enum ImGuiBackendFlags {
    None = 0,
    HasGamepad = 1 << 0,
    HasMouseCursors = 1 << 1,
    HasSetMousePos = 1 << 2,
    RendererHasVtxOffset = 1 << 3,
}

pub enum ImGuiCol {
    Text,
    TextDisabled,
    WindowBg,
    ChildBg,
    PopupBg,
    Border,
    BorderShadow,
    FrameBg,
    FrameBgHovered,
    FrameBgActive,
    TitleBg,
    TitleBgActive,
    TitleBgCollapsed,
    MenuBarBg,
    ScrollbarBg,
    ScrollbarGrab,
    ScrollbarGrabHovered,
    ScrollbarGrabActive,
    CheckMark,
    SliderGrab,
    SliderGrabActive,
    Button,
    ButtonHovered,
    ButtonActive,
    Header,
    HeaderHovered,
    HeaderActive,
    Separator,
    SeparatorHovered,
    SeparatorActive,
    ResizeGrip,
    ResizeGripHovered,
    ResizeGripActive,
    Tab,
    TabHovered,
    TabActive,
    TabUnfocused,
    TabUnfocusedActive,
    PlotLines,
    PlotLinesHovered,
    PlotHistogram,
    PlotHistogramHovered,
    TableHeaderBg,
    TableBorderStrong,
    TableBorderLight,
    TableRowBg,
    TableRowBgAlt,
    TextSelectedBg,
    DragDropTarget,
    NavHighlight,
    NavWindowingHighlight,
    NavWindowingDimBg,
    ModalWindowDimBg,
    COUNT,
}

pub enum ImGuiStyleVar {
    Alpha,
    DisabledAlpha,
    WindowPadding,
    WindowRounding,
    WindowBorderSize,
    WindowMinSize,
    WindowTitleAlign,
    ChildRounding,
    ChildBorderSize,
    PopupRounding,
    PopupBorderSize,
    FramePadding,
    FrameRounding,
    FrameBorderSize,
    ItemSpacing,
    ItemInnerSpacing,
    IndentSpacing,
    CellPadding,
    ScrollbarSize,
    ScrollbarRounding,
    GrabMinSize,
    GrabRounding,
    TabRounding,
    ButtonTextAlign,
    SelectableTextAlign,
    COUNT,
}

pub enum ImGuiButtonFlags {
    None,
    MouseButtonLeft = 1 << 0,
    MouseButtonRight = 1 << 1,
    MouseButtonMiddle = 1 << 2,
}

pub enum ImGuiColorEditFlags {
    None = 0,
    NoAlpha = 1 << 1,
    NoPicker = 1 << 2,
    NoOptions = 1 << 3,
    NoSmallPreview = 1 << 4,
    NoInputs = 1 << 5,
    NoTooltip = 1 << 6,
    NoLabel = 1 << 7,
    NoSidePreview = 1 << 8,
    NoDragDrop = 1 << 9,
    NoBorder = 1 << 10,
    AlphaBar = 1 << 16,
    AlphaPreview = 1 << 17,
    AlphaPreviewHalf = 1 << 18,
    HDR = 1 << 19,
    DisplayRGB = 1 << 20,
    DisplayHSV = 1 << 21,
    DisplayHex = 1 << 22,
    Uint8 = 1 << 23,
    Float = 1 << 24,
    PickerHueBar = 1 << 25,
    PickerHueWheel = 1 << 26,
    InputRGB = 1 << 27,
    InputHSV = 1 << 28,
}

pub enum ImGuiSliderFlags {
    None = 0,
    AlwaysClamp = 1 << 4,
    Logarithmic = 1 << 5,
    NoRoundToFormat = 1 << 6,
    NoInput = 1 << 7,
    InvalidMask_ = 0x7000000F,
}

pub enum ImGuiMouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    COUNT = 5,
}

pub enum ImGuiMouseCursor {
    None = -1,
    Arrow = 0,
    TextInput,
    ResizeAll,
    ResizeNS,
    ResizeEW,
    ResizeNESW,
    ResizeNWSE,
    Hand,
    NotAllowed,
    COUNT,
}

pub enum ImGuiCond {
    None = 0,
    Always = 1 << 0,
    Once = 1 << 1,
    FirstUseEver = 1 << 2,
    Appearing = 1 << 3,
}

#[macro_export]
macro_rules! to_flag {
    ($flag:expr) => {
        $flag as i32
    };
    ($flag:expr, $default:expr) => {
        $flag.map_or($default as i32, |f| f as i32)
    };
}

pub fn begin(name: &str, open: *mut bool, flags: ImGuiWindowFlags) -> bool {
    cxx::let_cxx_string!(name = name);
    unsafe { imgui_rs::Begin(&name, open, flags as i32) }
}

pub fn end() {
    imgui_rs::End()
}

pub fn begin_child(
    name: &str,
    size: ImVec2,
    child_flags: Option<ImGuiCond>,
    window_flags: Option<ImGuiCond>,
) -> bool {
    cxx::let_cxx_string!(name = name);
    imgui_rs::BeginChild(
        &name,
        &size.to_vec(),
        to_flag!(child_flags, ImGuiCond::None),
        to_flag!(window_flags, ImGuiCond::None),
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

pub fn is_window_focused(flags: ImGuiFocusedFlags) -> bool {
    imgui_rs::IsWindowFocused(to_flag!(flags))
}

pub fn is_window_hovered(flags: ImGuiHoveredFlags) -> bool {
    imgui_rs::IsWindowHovered(to_flag!(flags))
}

pub fn get_window_pos() -> ImVec2 {
    ImVec2::new(imgui_rs::GetWindowPos())
}

pub fn get_window_size() -> ImVec2 {
    ImVec2::new(imgui_rs::GetWindowSize())
}

pub fn get_window_width() -> f32 {
    imgui_rs::GetWindowWidth()
}

pub fn get_window_height() -> f32 {
    imgui_rs::GetWindowHeight()
}

pub fn set_next_window_pos(pos: ImVec2, cond: Option<ImGuiCond>, pivot: Option<ImVec2>) {
    imgui_rs::SetNextWindowPos(
        &pos.to_vec(),
        to_flag!(cond, ImGuiCond::None),
        &pivot.map_or([0.0, 0.0], |p| p.to_vec()),
    )
}

pub fn set_next_window_size(size: ImVec2, cond: Option<ImGuiCond>) {
    imgui_rs::SetNextWindowSize(&size.to_vec(), to_flag!(cond, ImGuiCond::None))
}

pub fn set_next_window_size_constraints(size_min: ImVec2, size_max: ImVec2) {
    imgui_rs::SetNextWindowSizeConstraints(&size_min.to_vec(), &size_max.to_vec())
}

pub fn set_next_window_content_size(size: ImVec2) {
    imgui_rs::SetNextWindowContentSize(&size.to_vec())
}

pub fn set_next_window_collapsed(collapsed: bool, cond: Option<ImGuiCond>) {
    imgui_rs::SetNextWindowCollapsed(collapsed, to_flag!(cond, ImGuiCond::None))
}

pub fn set_next_window_focus() {
    imgui_rs::SetNextWindowFocus()
}

pub fn set_next_window_bg_alpha(alpha: f32) {
    imgui_rs::SetNextWindowBgAlpha(alpha)
}

pub fn set_window_pos(pos: ImVec2, cond: Option<ImGuiCond>) {
    imgui_rs::SetWindowPos(&pos.to_vec(), to_flag!(cond, ImGuiCond::None))
}

pub fn set_window_size(size: ImVec2, cond: Option<ImGuiCond>) {
    imgui_rs::SetWindowSize(&size.to_vec(), to_flag!(cond, ImGuiCond::None))
}

pub fn set_window_collapsed(collapsed: bool, cond: Option<ImGuiCond>) {
    imgui_rs::SetWindowCollapsed(collapsed, to_flag!(cond, ImGuiCond::None))
}

pub fn set_window_focus() {
    imgui_rs::SetWindowFocus()
}

pub fn set_window_font_scale(scale: f32) {
    imgui_rs::SetWindowFontScale(scale)
}

pub fn set_window_pos_by_name(name: &str, pos: ImVec2, cond: Option<ImGuiCond>) {
    cxx::let_cxx_string!(name = name);
    imgui_rs::SetWindowPosByName(&name, &pos.to_vec(), to_flag!(cond, ImGuiCond::None))
}

pub fn set_window_size_by_name(name: &str, size: ImVec2, cond: Option<ImGuiCond>) {
    cxx::let_cxx_string!(name = name);
    imgui_rs::SetWindowSizeByName(&name, &size.to_vec(), to_flag!(cond, ImGuiCond::None))
}

pub fn set_window_collapsed_by_name(name: &str, collapsed: bool, cond: Option<ImGuiCond>) {
    cxx::let_cxx_string!(name = name);
    imgui_rs::SetWindowCollapsedByName(&name, collapsed, to_flag!(cond, ImGuiCond::None))
}

pub fn set_window_focus_by_name(name: &str) {
    cxx::let_cxx_string!(name = name);
    imgui_rs::SetWindowFocusByName(&name)
}

pub fn get_scroll() -> ImVec2 {
    ImVec2::new([imgui_rs::GetScrollX(), imgui_rs::GetScrollY()])
}

pub fn get_scroll_x() -> f32 {
    imgui_rs::GetScrollX()
}

pub fn get_scroll_y() -> f32 {
    imgui_rs::GetScrollY()
}

pub fn set_scroll(scroll: ImVec2) {
    imgui_rs::SetScrollX(scroll.x);
    imgui_rs::SetScrollY(scroll.y);
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

pub fn pop_font() {
    imgui_rs::PopFont()
}

pub fn push_style_color(idx: ImGuiCol, col: ImRgba) {
    imgui_rs::PushStyleColor(to_flag!(idx), &col.to_vec())
}

pub fn pop_style_color(count: Option<i32>) {
    imgui_rs::PopStyleColor(count.unwrap_or(1))
}

pub fn push_style_var_float(idx: ImGuiStyleVar, val: f32) {
    imgui_rs::PushStyleVar(to_flag!(idx), &[val, -1.0])
}

pub fn push_style_var_vec(idx: ImGuiStyleVar, val: ImVec2) {
    imgui_rs::PushStyleVar(to_flag!(idx), &val.to_vec())
}

pub fn pop_style_var(count: Option<i32>) {
    imgui_rs::PopStyleVar(count.unwrap_or(1))
}

pub fn push_item_width(item_width: f32) {
    imgui_rs::PushItemWidth(item_width)
}

pub fn pop_item_width() {
    imgui_rs::PopItemWidth()
}

pub fn set_next_item_width(item_width: f32) {
    imgui_rs::SetNextItemWidth(item_width)
}

pub fn calc_item_width() -> f32 {
    imgui_rs::CalcItemWidth()
}

pub fn push_text_wrap_pos(wrap_pos_x: Option<f32>) {
    imgui_rs::PushTextWrapPos(wrap_pos_x.unwrap_or(0.0))
}

pub fn pop_text_wrap_pos() {
    imgui_rs::PopTextWrapPos()
}

pub fn get_font_size() -> f32 {
    imgui_rs::GetFontSize()
}

pub fn get_font_tex_uv_white_pixel() -> ImVec2 {
    ImVec2::new(imgui_rs::GetFontTextUvWhitePixel())
}

pub fn get_color_u32(col: ImVec4) -> u32 {
    imgui_rs::GetColorU32(&col.to_vec())
}

pub fn get_color_u32_from_style(idx: ImGuiCol, alpha_mul: Option<f32>) -> u32 {
    imgui_rs::GetColorU32FromStyle(to_flag!(idx), alpha_mul.unwrap_or(1.0))
}

pub fn get_style_color_vec4(idx: ImGuiCol) -> ImVec4 {
    ImVec4::new(imgui_rs::GetStyleColorVec4(to_flag!(idx)))
}

pub fn separator() {
    imgui_rs::Separator()
}

pub fn same_line(offset_from_start_x: Option<f32>, spacing: Option<f32>) {
    imgui_rs::SameLine(offset_from_start_x.unwrap_or(0.0), spacing.unwrap_or(-1.0))
}

pub fn new_line() {
    imgui_rs::NewLine()
}

pub fn spacing() {
    imgui_rs::Spacing()
}

pub fn dummy(size: ImVec2) {
    imgui_rs::Dummy(&size.to_vec())
}

pub fn indent(index_w: Option<f32>) {
    imgui_rs::Indent(index_w.unwrap_or(0.0))
}

pub fn unindent(index_w: Option<f32>) {
    imgui_rs::Unindent(index_w.unwrap_or(0.0))
}

pub fn begin_group() {
    imgui_rs::BeginGroup()
}

pub fn end_group() {
    imgui_rs::EndGroup()
}

pub fn get_cursor_pos() -> ImVec2 {
    ImVec2::new(imgui_rs::GetCursorPos())
}

pub fn get_cursor_pos_x() -> f32 {
    imgui_rs::GetCursorPosX()
}

pub fn get_cursor_pos_y() -> f32 {
    imgui_rs::GetCursorPosY()
}

pub fn set_cursor_pos(pos: ImVec2) {
    imgui_rs::SetCursorPos(&pos.to_vec())
}

pub fn set_cursor_pos_x(x: f32) {
    imgui_rs::SetCursorPosX(x)
}

pub fn set_cursor_pos_y(y: f32) {
    imgui_rs::SetCursorPosY(y)
}

pub fn get_cursor_start_pos() -> ImVec2 {
    ImVec2::new(imgui_rs::GetCursorStartPos())
}

pub fn get_cursor_screen_pos() -> ImVec2 {
    ImVec2::new(imgui_rs::GetCursorScreenPos())
}

pub fn set_cursor_screen_pos(pos: ImVec2) {
    imgui_rs::SetCursorScreenPos(&pos.to_vec())
}

pub fn align_text_to_frame_padding() {
    imgui_rs::AlignTextToFramePadding()
}

pub fn get_text_line_height() -> f32 {
    imgui_rs::GetTextLineHeight()
}

pub fn get_text_line_height_with_spacing() -> f32 {
    imgui_rs::GetTextLineHeightWithSpacing()
}

pub fn get_frame_height() -> f32 {
    imgui_rs::GetFrameHeight()
}

pub fn get_frame_height_with_spacing() -> f32 {
    imgui_rs::GetFrameHeightWithSpacing()
}

pub fn push_id(id: i32) {
    imgui_rs::PushID(id)
}

pub fn push_id_str(str_id: &str) {
    cxx::let_cxx_string!(str_id = str_id);
    imgui_rs::PushIDString(&str_id)
}

pub fn pop_id() {
    imgui_rs::PopID()
}

pub fn get_id(id: i32) -> u32 {
    imgui_rs::GetID(id)
}

pub fn get_id_str(str_id: &str) -> u32 {
    cxx::let_cxx_string!(str_id = str_id);
    imgui_rs::GetIDFromString(&str_id)
}

pub fn text_unformatted(str: &str) {
    cxx::let_cxx_string!(str = str);
    imgui_rs::TextUnformatted(&str)
}

pub fn text(fmt: &str, _args: Vec<&str>) {
    cxx::let_cxx_string!(fmt = fmt);
    let cxx_args = CxxVector::new();
    // TODO: fill cxx_args
    imgui_rs::Text(&fmt, &cxx_args)
}

pub fn text_colored(col: ImVec4, fmt: &str, _args: Vec<&str>) {
    cxx::let_cxx_string!(fmt = fmt);
    let cxx_args = CxxVector::new();
    imgui_rs::TextColored(&col.to_vec(), &fmt, &cxx_args)
}

pub fn text_disabled(fmt: &str, _args: Vec<&str>) {
    cxx::let_cxx_string!(fmt = fmt);
    let cxx_args = CxxVector::new();
    imgui_rs::TextDisabled(&fmt, &cxx_args)
}

pub fn text_wrapped(fmt: &str, _args: Vec<&str>) {
    cxx::let_cxx_string!(fmt = fmt);
    let cxx_args = CxxVector::new();
    imgui_rs::TextWrapped(&fmt, &cxx_args)
}

pub fn label_text(label: &str, fmt: &str, _args: Vec<&str>) {
    cxx::let_cxx_string!(label = label);
    cxx::let_cxx_string!(fmt = fmt);
    let cxx_args = CxxVector::new();
    imgui_rs::LabelText(&label, &fmt, &cxx_args)
}

pub fn bullet_text(fmt: &str, _args: Vec<&str>) {
    cxx::let_cxx_string!(fmt = fmt);
    let cxx_args = CxxVector::new();
    imgui_rs::BulletText(&fmt, &cxx_args)
}

pub fn separator_text(fmt: &str) {
    cxx::let_cxx_string!(fmt = fmt);
    imgui_rs::SeparatorText(&fmt)
}

pub fn button(label: &str, size: Option<ImVec2>) -> bool {
    cxx::let_cxx_string!(label = label);
    imgui_rs::Button(&label, &size.unwrap_or(ImVec2::default()).to_vec())
}

pub fn small_button(label: &str) -> bool {
    cxx::let_cxx_string!(label = label);
    imgui_rs::SmallButton(&label)
}

pub fn invisible_button(str_id: &str, size: ImVec2, flags: Option<ImGuiButtonFlags>) -> bool {
    cxx::let_cxx_string!(str_id = str_id);
    imgui_rs::InvisibleButton(
        &str_id,
        &size.to_vec(),
        to_flag!(flags, ImGuiButtonFlags::None),
    )
}

pub fn arrow_button(str_id: &str, dir: ImGuiDir) -> bool {
    cxx::let_cxx_string!(str_id = str_id);
    imgui_rs::ArrowButton(&str_id, to_flag!(dir))
}

pub fn checkbox(label: &str, v: *mut bool) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe { imgui_rs::Checkbox(&label, v) }
}

pub fn checkbox_flags(label: &str, flags: *mut i32, flags_value: i32) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe { imgui_rs::CheckboxFlags(&label, flags, flags_value) }
}

pub fn radio_button_bool(label: &str, active: bool) -> bool {
    cxx::let_cxx_string!(label = label);
    imgui_rs::RadioButtonBool(&label, active)
}

pub fn radio_button(label: &str, v: *mut i32, v_button: i32) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe { imgui_rs::RadioButton(&label, v, v_button) }
}

pub fn progress_bar(fraction: f32, size_arg: Option<ImVec2>, overlay: Option<&str>) {
    let overlay = overlay.unwrap_or("");
    cxx::let_cxx_string!(overlay = overlay);
    imgui_rs::ProgressBar(
        fraction,
        &size_arg.unwrap_or(ImVec2::default()).to_vec(),
        &overlay,
    )
}

pub fn bullet() {
    imgui_rs::Bullet()
}

pub fn begin_combo(label: &str, preview_value: &str, flags: Option<ImGuiComboFlags>) -> bool {
    cxx::let_cxx_string!(label = label);
    cxx::let_cxx_string!(preview_value = preview_value);
    imgui_rs::BeginCombo(
        &label,
        &preview_value,
        to_flag!(flags, ImGuiComboFlags::None),
    )
}

pub fn end_combo() {
    imgui_rs::EndCombo()
}

pub fn combo(label: &str, current_item: *mut i32, items: Vec<&str>, height_in_items: i32) -> bool {
    cxx::let_cxx_string!(label = label);
    let items = items.iter().map(|s| s.to_string()).collect();
    unsafe { imgui_rs::Combo(&label, current_item, &items, height_in_items) }
}

pub fn drag_float(
    label: &str,
    v: *mut f32,
    v_speed: f32,
    v_min: f32,
    v_max: f32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%.3f");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::DragFloat(
            &label,
            v,
            v_speed,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn drag_float2(
    label: &str,
    v: *mut [f32; 2],
    v_speed: f32,
    v_min: f32,
    v_max: f32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%.3f");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::DragFloat2(
            &label,
            v,
            v_speed,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn drag_float3(
    label: &str,
    v: *mut [f32; 3],
    v_speed: f32,
    v_min: f32,
    v_max: f32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%.3f");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::DragFloat3(
            &label,
            v,
            v_speed,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn drag_float4(
    label: &str,
    v: *mut [f32; 4],
    v_speed: f32,
    v_min: f32,
    v_max: f32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%.3f");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::DragFloat4(
            &label,
            v,
            v_speed,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn drag_float_range2(
    label: &str,
    v_current_min: *mut f32,
    v_current_max: *mut f32,
    v_speed: f32,
    v_min: f32,
    v_max: f32,
    format: Option<&str>,
    format_max: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%.3f");
    cxx::let_cxx_string!(format = format);
    let format_max = format_max.unwrap_or("%.3f");
    cxx::let_cxx_string!(format_max = format_max);
    unsafe {
        imgui_rs::DragFloatRange2(
            &label,
            v_current_min,
            v_current_max,
            v_speed,
            v_min,
            v_max,
            &format,
            &format_max,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn drag_int(
    label: &str,
    v: *mut i32,
    v_speed: f32,
    v_min: i32,
    v_max: i32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%d");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::DragInt(
            &label,
            v,
            v_speed,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn drag_int2(
    label: &str,
    v: *mut [i32; 2],
    v_speed: f32,
    v_min: i32,
    v_max: i32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%d");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::DragInt2(
            &label,
            v,
            v_speed,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn drag_int3(
    label: &str,
    v: *mut [i32; 3],
    v_speed: f32,
    v_min: i32,
    v_max: i32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%d");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::DragInt3(
            &label,
            v,
            v_speed,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn drag_int4(
    label: &str,
    v: *mut [i32; 4],
    v_speed: f32,
    v_min: i32,
    v_max: i32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%d");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::DragInt4(
            &label,
            v,
            v_speed,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn drag_int_range2(
    label: &str,
    v_current_min: *mut i32,
    v_current_max: *mut i32,
    v_speed: f32,
    v_min: i32,
    v_max: i32,
    format: Option<&str>,
    format_max: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%d");
    cxx::let_cxx_string!(format = format);
    let format_max = format_max.unwrap_or("%d");
    cxx::let_cxx_string!(format_max = format_max);
    unsafe {
        imgui_rs::DragIntRange2(
            &label,
            v_current_min,
            v_current_max,
            v_speed,
            v_min,
            v_max,
            &format,
            &format_max,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn slider_float(
    label: &str,
    v: *mut f32,
    v_min: f32,
    v_max: f32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%.3f");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::SliderFloat(
            &label,
            v,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn slider_float2(
    label: &str,
    v: *mut [f32; 2],
    v_min: f32,
    v_max: f32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%.3f");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::SliderFloat2(
            &label,
            v,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn slider_float3(
    label: &str,
    v: *mut [f32; 3],
    v_min: f32,
    v_max: f32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%.3f");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::SliderFloat3(
            &label,
            v,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn slider_float4(
    label: &str,
    v: *mut [f32; 4],
    v_min: f32,
    v_max: f32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%.3f");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::SliderFloat4(
            &label,
            v,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn slider_angle(
    label: &str,
    v_rad: *mut f32,
    v_degrees_min: f32,
    v_degrees_max: f32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%.0f deg");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::SliderAngle(
            &label,
            v_rad,
            v_degrees_min,
            v_degrees_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn slider_int(
    label: &str,
    v: *mut i32,
    v_min: i32,
    v_max: i32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%d");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::SliderInt(
            &label,
            v,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn slider_int2(
    label: &str,
    v: *mut [i32; 2],
    v_min: i32,
    v_max: i32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%d");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::SliderInt2(
            &label,
            v,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn slider_int3(
    label: &str,
    v: *mut [i32; 3],
    v_min: i32,
    v_max: i32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%d");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::SliderInt3(
            &label,
            v,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn slider_int4(
    label: &str,
    v: *mut [i32; 4],
    v_min: i32,
    v_max: i32,
    format: Option<&str>,
    flags: Option<ImGuiSliderFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let format = format.unwrap_or("%d");
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::SliderInt4(
            &label,
            v,
            v_min,
            v_max,
            &format,
            to_flag!(flags, ImGuiSliderFlags::None),
        )
    }
}

pub fn input_text(label: &str, buf: &mut String, flags: Option<ImGuiInputTextFlags>) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe {
        imgui_rs::InputText(
            &label,
            buf.as_mut_vec(),
            to_flag!(flags, ImGuiInputTextFlags::None),
        )
    }
}

pub fn input_text_with_hint(
    label: &str,
    hint: &str,
    buf: &mut String,
    flags: Option<ImGuiInputTextFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    cxx::let_cxx_string!(hint = hint);
    unsafe {
        imgui_rs::InputTextWithHint(
            &label,
            &hint,
            buf.as_mut_vec(),
            to_flag!(flags, ImGuiInputTextFlags::None),
        )
    }
}

pub fn input_float(
    label: &str,
    v: *mut f32,
    step: f32,
    step_fast: f32,
    format: &str,
    flags: Option<ImGuiInputTextFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::InputFloat(
            &label,
            v,
            step,
            step_fast,
            &format,
            to_flag!(flags, ImGuiInputTextFlags::None),
        )
    }
}

pub fn input_float2(
    label: &str,
    v: *mut [f32; 2],
    format: &str,
    flags: Option<ImGuiInputTextFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::InputFloat2(
            &label,
            v,
            &format,
            to_flag!(flags, ImGuiInputTextFlags::None),
        )
    }
}

pub fn input_float3(
    label: &str,
    v: *mut [f32; 3],
    format: &str,
    flags: Option<ImGuiInputTextFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::InputFloat3(
            &label,
            v,
            &format,
            to_flag!(flags, ImGuiInputTextFlags::None),
        )
    }
}

pub fn input_float4(
    label: &str,
    v: *mut [f32; 4],
    format: &str,
    flags: Option<ImGuiInputTextFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::InputFloat4(
            &label,
            v,
            &format,
            to_flag!(flags, ImGuiInputTextFlags::None),
        )
    }
}

pub fn input_int(
    label: &str,
    v: *mut i32,
    step: i32,
    step_fast: i32,
    flags: Option<ImGuiInputTextFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe {
        imgui_rs::InputInt(
            &label,
            v,
            step,
            step_fast,
            to_flag!(flags, ImGuiInputTextFlags::None),
        )
    }
}

pub fn input_int2(label: &str, v: *mut [i32; 2], flags: Option<ImGuiInputTextFlags>) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe { imgui_rs::InputInt2(&label, v, to_flag!(flags, ImGuiInputTextFlags::None)) }
}

pub fn input_int3(label: &str, v: *mut [i32; 3], flags: Option<ImGuiInputTextFlags>) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe { imgui_rs::InputInt3(&label, v, to_flag!(flags, ImGuiInputTextFlags::None)) }
}

pub fn input_int4(label: &str, v: *mut [i32; 4], flags: Option<ImGuiInputTextFlags>) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe { imgui_rs::InputInt4(&label, v, to_flag!(flags, ImGuiInputTextFlags::None)) }
}

pub fn input_double(
    label: &str,
    v: &mut f64,
    step: f64,
    step_fast: f64,
    format: &str,
    flags: Option<ImGuiInputTextFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    cxx::let_cxx_string!(format = format);
    unsafe {
        imgui_rs::InputDouble(
            &label,
            v,
            step,
            step_fast,
            &format,
            to_flag!(flags, ImGuiInputTextFlags::None),
        )
    }
}

pub fn color_edit3(label: &str, col: &mut ImRgb, flags: Option<ImGuiColorEditFlags>) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe {
        imgui_rs::ColorEdit3(
            &label,
            // I dunno if this will actually work lol
            &mut col.to_vec(),
            to_flag!(flags, ImGuiColorEditFlags::None),
        )
    }
}

pub fn color_edit4(label: &str, col: &mut ImRgba, flags: Option<ImGuiColorEditFlags>) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe {
        imgui_rs::ColorEdit4(
            &label,
            &mut col.to_vec(),
            to_flag!(flags, ImGuiColorEditFlags::None),
        )
    }
}

pub fn color_picker3(label: &str, col: &mut ImRgb, flags: Option<ImGuiColorEditFlags>) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe {
        imgui_rs::ColorPicker3(
            &label,
            &mut col.to_vec(),
            to_flag!(flags, ImGuiColorEditFlags::None),
        )
    }
}

pub fn color_picker4(
    label: &str,
    col: &mut ImRgba,
    flags: Option<ImGuiColorEditFlags>,
    ref_col: ImVec4,
) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe {
        imgui_rs::ColorPicker4(
            &label,
            &mut col.to_vec(),
            to_flag!(flags, ImGuiColorEditFlags::None),
            &ref_col.to_vec(),
        )
    }
}

pub fn color_button(
    desc_id: &str,
    col: &mut ImRgba,
    flags: Option<ImGuiColorEditFlags>,
    size: ImVec2,
) -> bool {
    cxx::let_cxx_string!(desc_id = desc_id);
    unsafe {
        imgui_rs::ColorButton(
            &desc_id,
            &mut col.to_vec(),
            to_flag!(flags, ImGuiColorEditFlags::None),
            &size.to_vec(),
        )
    }
}

pub fn set_color_edit_options(flags: ImGuiColorEditFlags) {
    imgui_rs::SetColorEditOptions(to_flag!(flags))
}

pub fn tree_node(label: &str) -> bool {
    cxx::let_cxx_string!(label = label);
    imgui_rs::TreeNode(&label)
}

pub fn tree_node_id(str_id: &str, fmt: &str) -> bool {
    cxx::let_cxx_string!(str_id = str_id);
    cxx::let_cxx_string!(fmt = fmt);
    imgui_rs::TreeNodeWithId(&str_id, &fmt)
}

pub fn tree_node_ex(label: &str, flags: Option<ImGuiTreeNodeFlags>) -> bool {
    cxx::let_cxx_string!(label = label);
    imgui_rs::TreeNodeEx(&label, to_flag!(flags, ImGuiTreeNodeFlags::None))
}

pub fn tree_node_ex_id(str_id: &str, flags: ImGuiTreeNodeFlags, fmt: &str) -> bool {
    cxx::let_cxx_string!(str_id = str_id);
    cxx::let_cxx_string!(fmt = fmt);
    imgui_rs::TreeNodeExWithId(&str_id, to_flag!(flags), &fmt)
}

pub fn tree_push(str_id: &str) {
    cxx::let_cxx_string!(str_id = str_id);
    imgui_rs::TreePush(&str_id)
}

pub fn tree_pop() {
    imgui_rs::TreePop()
}

pub fn get_tree_node_to_label_spacing() -> f32 {
    imgui_rs::GetTreeNodeToLabelSpacing()
}

pub fn collapsing_header(label: &str, flags: Option<ImGuiTreeNodeFlags>) -> bool {
    cxx::let_cxx_string!(label = label);
    imgui_rs::CollapsingHeader(&label, to_flag!(flags, ImGuiTreeNodeFlags::None))
}

pub fn collapsing_header_ctrl(
    label: &str,
    open: *mut bool,
    flags: Option<ImGuiTreeNodeFlags>,
) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe {
        imgui_rs::CollapsingHeaderCtrl(&label, open, to_flag!(flags, ImGuiTreeNodeFlags::None))
    }
}

pub fn set_next_item_open(is_open: bool, cond: Option<ImGuiCond>) {
    imgui_rs::SetNextItemOpen(is_open, to_flag!(cond, ImGuiCond::None))
}

pub fn selectable(
    label: &str,
    selected: *mut bool,
    flags: Option<ImGuiSelectableFlags>,
    size: ImVec2,
) -> bool {
    cxx::let_cxx_string!(label = label);
    unsafe {
        imgui_rs::Selectable(
            &label,
            selected,
            to_flag!(flags, ImGuiSelectableFlags::None),
            &size.to_vec(),
        )
    }
}

pub fn list_box(
    label: &str,
    current_item: *mut i32,
    items: Vec<&str>,
    height_in_items: i32,
) -> bool {
    cxx::let_cxx_string!(label = label);
    let items = items.iter().map(|s| s.to_string()).collect();
    unsafe { imgui_rs::ListBox(&label, current_item, &items, height_in_items) }
}

pub fn begin_main_menu_bar() -> bool {
    imgui_rs::BeginMainMenuBar()
}

pub fn end_main_menu_bar() {
    imgui_rs::EndMainMenuBar()
}

pub fn begin_menu_bar() -> bool {
    imgui_rs::BeginMenuBar()
}

pub fn end_menu_bar() {
    imgui_rs::EndMenuBar()
}

pub fn begin_menu(label: &str, enabled: bool) -> bool {
    cxx::let_cxx_string!(label = label);
    imgui_rs::BeginMenu(&label, enabled)
}

pub fn end_menu() {
    imgui_rs::EndMenu()
}

pub fn menu_item(label: &str, shortcut: Option<&str>, selected: bool, enabled: bool) -> bool {
    cxx::let_cxx_string!(label = label);
    let shortcut = shortcut.unwrap_or("");
    cxx::let_cxx_string!(shortcut = shortcut);
    imgui_rs::MenuItem(&label, &shortcut, selected, enabled)
}

pub fn begin_tooltip() {
    imgui_rs::BeginTooltip()
}

pub fn end_tooltip() {
    imgui_rs::EndTooltip()
}

pub fn set_tooltip(fmt: &str, _args: Vec<&str>) {
    cxx::let_cxx_string!(fmt = fmt);
    imgui_rs::SetTooltip(&fmt)
}

pub fn begin_popup(str_id: &str, flags: Option<ImGuiWindowFlags>) -> bool {
    cxx::let_cxx_string!(str_id = str_id);
    imgui_rs::BeginPopup(&str_id, to_flag!(flags, ImGuiWindowFlags::None))
}

pub fn begin_popup_modal(name: &str, open: *mut bool, flags: Option<ImGuiWindowFlags>) -> bool {
    cxx::let_cxx_string!(name = name);
    unsafe { imgui_rs::BeginPopupModal(&name, open, to_flag!(flags, ImGuiWindowFlags::None)) }
}

pub fn end_popup() {
    imgui_rs::EndPopup()
}

pub fn open_popup(str_id: &str, flags: Option<ImGuiPopupFlags>) {
    cxx::let_cxx_string!(str_id = str_id);
    imgui_rs::OpenPopup(
        &str_id,
        to_flag!(flags, ImGuiPopupFlags::NoneOrMouseButtonLeft),
    )
}

pub fn open_popup_on_item_click(str_id: &str, flags: Option<ImGuiPopupFlags>) {
    cxx::let_cxx_string!(str_id = str_id);
    imgui_rs::OpenPopupOnItemClick(&str_id, to_flag!(flags, ImGuiPopupFlags::MouseButtonRight))
}

pub fn close_current_popup() {
    imgui_rs::CloseCurrentPopup()
}

pub fn begin_popup_context_item(str_id: &str, flags: Option<ImGuiPopupFlags>) -> bool {
    cxx::let_cxx_string!(str_id = str_id);
    imgui_rs::BeginPopupContextItem(&str_id, to_flag!(flags, ImGuiPopupFlags::MouseButtonRight))
}

pub fn begin_popup_context_window(str_id: &str, flags: Option<ImGuiPopupFlags>) -> bool {
    cxx::let_cxx_string!(str_id = str_id);
    imgui_rs::BeginPopupContextWindow(&str_id, to_flag!(flags, ImGuiPopupFlags::MouseButtonRight))
}

pub fn begin_popup_context_void(str_id: &str, flags: Option<ImGuiPopupFlags>) -> bool {
    cxx::let_cxx_string!(str_id = str_id);
    imgui_rs::BeginPopupContextVoid(&str_id, to_flag!(flags, ImGuiPopupFlags::MouseButtonRight))
}

pub fn is_popup_open(str_id: &str, flags: Option<ImGuiPopupFlags>) -> bool {
    cxx::let_cxx_string!(str_id = str_id);
    imgui_rs::IsPopupOpen(
        &str_id,
        to_flag!(flags, ImGuiPopupFlags::NoneOrMouseButtonLeft),
    )
}

pub fn begin_table(
    str_id: &str,
    column_count: i32,
    flags: Option<ImGuiTableFlags>,
    outer_size: Option<ImVec2>,
    inner_width: Option<f32>,
) -> bool {
    cxx::let_cxx_string!(str_id = str_id);
    imgui_rs::BeginTable(
        &str_id,
        column_count,
        to_flag!(flags, ImGuiTableFlags::None),
        &outer_size.unwrap_or(ImVec2::new([0.0, 0.0])).to_vec(),
        inner_width.unwrap_or(0.0),
    )
}

pub fn end_table() {
    imgui_rs::EndTable()
}

pub fn table_next_row(row_flags: Option<ImGuiTableRowFlags>, min_row_height: Option<f32>) {
    imgui_rs::TableNextRow(
        to_flag!(row_flags, ImGuiTableRowFlags::None),
        min_row_height.unwrap_or(0.0),
    )
}

pub fn table_next_column() -> bool {
    imgui_rs::TableNextColumn()
}

pub fn table_set_column_index(column_n: i32) -> bool {
    imgui_rs::TableSetColumnIndex(column_n)
}

pub fn table_setup_column(
    label: &str,
    flags: Option<ImGuiTableColumnFlags>,
    init_width_or_weight: Option<f32>,
    user_id: Option<u32>,
) {
    cxx::let_cxx_string!(label = label);
    imgui_rs::TableSetupColumn(
        &label,
        to_flag!(flags, ImGuiTableColumnFlags::None),
        init_width_or_weight.unwrap_or(0.0),
        user_id.unwrap_or(0),
    )
}

pub fn table_setup_scroll_freeze(left_columns: i32, right_columns: i32) {
    imgui_rs::TableSetupScrollFreeze(left_columns, right_columns)
}

pub fn table_headers_row() {
    imgui_rs::TableHeadersRow()
}

pub fn table_header(label: &str) {
    cxx::let_cxx_string!(label = label);
    imgui_rs::TableHeader(&label)
}

pub fn table_get_sort_specs() -> *const ImGuiTableSortSpecs {
    imgui_rs::TableGetSortSpecs() as *const ImGuiTableSortSpecs
}

pub fn table_get_column_count() -> i32 {
    imgui_rs::TableGetColumnCount()
}

pub fn table_get_column_index() -> i32 {
    imgui_rs::TableGetColumnIndex()
}
