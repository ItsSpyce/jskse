#pragma once
#include "bridge/cosave.rs.h"

#define RIMGUI_VEC rust::Vec<float_t>

namespace rimgui {

/// Custom ImGui Builder functions/structs


// Built-in ImGui functions

bool Begin(rust::String name);
void End();
bool BeginChild(rust::String id, const RIMGUI_VEC &size, int child_flags,
                int window_flags);
void EndChild();
bool IsWindowAppearing();
bool IsWindowCollapsed();
bool IsWindowFocused(int flags);
bool IsWindowHovered(int flags);
RIMGUI_VEC GetWindowPos();
RIMGUI_VEC GetWindowSize();
float GetWindowWidth();
float GetWindowHeight();
void SetNextWindowPos(RIMGUI_VEC &size, int cond, RIMGUI_VEC &pivot);
void SetNextWindowSize(RIMGUI_VEC &size, int cond);
void SetNextWindowSizeConstraints(RIMGUI_VEC &size_min, RIMGUI_VEC &size_max);
void SetNextWindowContentSize(RIMGUI_VEC &size);
void SetNextWindowCollapsed(bool collapsed, int cond);
void SetNextWindowFocus();
void SetNextWindowScroll(RIMGUI_VEC &scroll);
void SetNextWindowBgAlpha(float alpha);
void SetWindowPos(RIMGUI_VEC &pos, int cond);
void SetWindowSize(RIMGUI_VEC &size, int cond);
void SetWindowCollapsed(bool collapsed, int cond);
void SetWindowFocus();
void SetWindowFontScale(float scale);
void SetWindowPosByName(rust::String name, RIMGUI_VEC &pos, int cond);
void SetWindowSizeByName(rust::String name, RIMGUI_VEC &size, int cond);
void SetWindowCollapsedByName(rust::String name, bool collapsed, int cond);
void SetWindowFocusByName(rust::String name);

float GetScrollX();
float GetScrollY();
void SetScrollX(float scroll_x);
void SetScrollY(float scroll_y);
float GetScrollMaxX();
float GetScrollMaxY();
void SetScrollHereX(float center_x_ratio);
void SetScrollHereY(float center_y_ratio);
void SetScrollFromPosX(float local_x, float center_x_ratio);
void SetScrollFromPosY(float local_y, float center_y_ratio);

void PopFont();
void PushStyleColor(int idx, RIMGUI_VEC &col);
void PopStyleColor(int count);
void PushStyleVar(int index, RIMGUI_VEC &val);
void PopStyleVar(int count);
void PushItemFlag(int option, bool enabled);
void PopItemFlag();

void PushItemWidth(float item_width);
void PopItemWidth();
void SetNextItemWidth(float item_width);
float CalcItemWidth();
void PushTextWrapPos(float wrap_local_pos_x);
void PopTextWrapPos();

float GetFontSize();
RIMGUI_VEC GetFontTextUvWhitePixel();
uint32_t GetColorU32(RIMGUI_VEC &col);
RIMGUI_VEC GetStyleColorVec4(int idx);

RIMGUI_VEC GetCursorScreenPos();
void SetCursorScreenPos(RIMGUI_VEC &pos);
RIMGUI_VEC GetContentRegionAvail();
RIMGUI_VEC GetCursorPos();
float GetCursorPosX();
float GetCursorPosY();
void SetCursorPos(RIMGUI_VEC &pos);
void SetCursorPosX(float x);
void SetCursorPosY(float y);
RIMGUI_VEC GetCursorStartPos();

void Separator();
void SameLine(float offset_from_start_x, float spacing);
void NewLine();
void Spacing();
void Dummy(RIMGUI_VEC &size);
void Indent(float indent_w);
void Unindent(float indent_w);
void BeginGroup();
void EndGroup();
void AlignTextToFramePadding();
float GetTextLineHeight();
float GetTextLineHeightWithSpacing();
float GetFrameHeight();
float GetFrameHeightWithSpacing();

void PushIDString(rust::String &id);
void PushID(int id);
void PopID();
uint32_t GetIDFromString(rust::String &str);
uint32_t GetID(int id);

void TextUnformatted(rust::String &text, rust::String &text_end);
void Text(rust::String &text, ...);
void TextColored(RIMGUI_VEC &col, rust::String &text, ...);
void TextDisabled(rust::String &text, ...);
void TextWrapped(rust::String &text, ...);
void LabelText(rust::String &label, rust::String &text, ...);
void BulletText(rust::String &text, ...);
void SeparatorText(rust::String &text);

bool Button(rust::String &label, RIMGUI_VEC &size);
bool SmallButton(rust::String &label);
bool InvisibleButton(rust::String &str_id, RIMGUI_VEC &size, int flags);
bool ArrowButton(rust::String &str_id, int dir);
bool Checkbox(rust::String &label, bool &v);
bool CheckboxFlags(rust::String &label, int flags, int flags_value);
bool RadioButtonBool(rust::String &label, bool active);
bool RadioButton(rust::String &label, int &v, int v_button);
void ProgressBar(float fraction, RIMGUI_VEC &size_arg, rust::String &overlay);
void Bullet();
bool TextLink(rust::String &label);
void TextLinkOpenURL(rust::String &label, rust::String &url);

// TODO: images

bool BeginCombo(rust::String &label, rust::String &preview_value, int flags);
void EndCombo();
bool Combo(rust::String &label, int &current_item, rust::Vec<rust::String> &items,
           int items_count, int height_in_items);

bool DragFloat(rust::String &label, float &v, float v_speed, float v_min,
               float v_max, rust::String &format, float power);
bool DragFloat2(rust::String &label, RIMGUI_VEC &v, float v_speed, float v_min,
                float v_max, rust::String &format, float power);
bool DragFloat3(rust::String &label, RIMGUI_VEC &v, float v_speed, float v_min,
                float v_max, rust::String &format, float power);
bool DragFloat4(rust::String &label, RIMGUI_VEC &v, float v_speed, float v_min,
                float v_max, rust::String &format, float power);
bool DragFloatRange2(rust::String &label, float &v_current_min,
                     float &v_current_max, float v_speed, float v_min,
                     float v_max, rust::String &format_min,
                     rust::String &format_max, float power);
bool DragInt(rust::String &label, int &v, float v_speed, int v_min, int v_max,
             rust::String &format);
bool DragInt2(rust::String &label, RIMGUI_VEC &v, float v_speed, int v_min,
              int v_max, rust::String &format);
bool DragInt3(rust::String &label, RIMGUI_VEC &v, float v_speed, int v_min,
              int v_max, rust::String &format);
bool DragInt4(rust::String &label, RIMGUI_VEC &v, float v_speed, int v_min,
              int v_max, rust::String &format);
bool DragIntRange2(rust::String &label, int &v_current_min, int &v_current_max,
                   float v_speed, int v_min, int v_max,
                   rust::String &format_min, rust::String &format_max);

bool SliderFloat(rust::String &label, float &v, float v_min, float v_max,
                 rust::String &format, float power);
bool SliderFloat2(rust::String &label, RIMGUI_VEC &v, float v_min, float v_max,
                  rust::String &format, float power);
bool SliderFloat3(rust::String &label, RIMGUI_VEC &v, float v_min, float v_max,
                  rust::String &format, float power);
bool SliderFloat4(rust::String &label, RIMGUI_VEC &v, float v_min, float v_max,
                  rust::String &format, float power);
bool SliderAngle(rust::String &label, float &v_rad, float v_degrees_min,
                 float v_degrees_max, rust::String &format);
bool SliderInt(rust::String &label, int &v, int v_min, int v_max,
               rust::String &format);
bool SliderInt2(rust::String &label, RIMGUI_VEC &v, int v_min, int v_max,
                rust::String &format);
bool SliderInt3(rust::String &label, RIMGUI_VEC &v, int v_min, int v_max,
                rust::String &format);
bool SliderInt4(rust::String &label, RIMGUI_VEC &v, int v_min, int v_max,
                rust::String &format);

bool InputText(rust::String &label, rust::String &buf, int flags);
bool InputTextWithHint(rust::String &label, rust::String &hint,
                       rust::String &buf, int flags);
bool InputFloat(rust::String &label, float &v, float step, float step_fast,
                rust::String &format, int flags);
bool InputFloat2(rust::String &label, RIMGUI_VEC &v, rust::String &format,
                 int flags);
bool InputFloat3(rust::String &label, RIMGUI_VEC &v, rust::String &format,
                 int flags);
bool InputFloat4(rust::String &label, RIMGUI_VEC &v, rust::String &format,
                 int flags);
bool InputInt(rust::String &label, int &v, int step, int step_fast, int flags);
bool InputInt2(rust::String &label, RIMGUI_VEC &v, int flags);
bool InputInt3(rust::String &label, RIMGUI_VEC &v, int flags);
bool InputInt4(rust::String &label, RIMGUI_VEC &v, int flags);
bool InputDouble(rust::String &label, double &v, double step, double step_fast,
                 rust::String &format, int flags);

bool ColorEdit3(rust::String &label, RIMGUI_VEC &col, int flags);
bool ColorEdit4(rust::String &label, RIMGUI_VEC &col, int flags);
bool ColorPicker3(rust::String &label, RIMGUI_VEC &col, int flags);
bool ColorPicker4(rust::String &label, RIMGUI_VEC &col, int flags,
                  RIMGUI_VEC &ref_col);
bool ColorButton(rust::String &desc_id, RIMGUI_VEC &col, int flags,
                 RIMGUI_VEC &size);
void SetColorEditOptions(int flags);

bool TreeNode(rust::String &label);
bool TreeNodeWithId(rust::String &str_id, rust::String &fmt, ...);
bool TreeNodeEx(rust::String &label, int flags);
bool TreeNodeExWithId(rust::String &str_id, int flags, rust::String &fmt, ...);
void TreePush(rust::String &str_id);
void TreePop();
float GetTreeNodeToLabelSpacing();
bool CollapsingHeader(rust::String &label, int flags);
void SetNextItemOpen(bool is_open, int cond);
void SetNextItemStorageID(uint32_t id);

bool Selectable(rust::String &label, bool selected, int flags,
                RIMGUI_VEC &size);

// TODO: multi-selectable

bool BeginListBox(rust::String &label, RIMGUI_VEC &size);
void EndListBox();
bool ListBox(rust::String &label, int &current_item,
             rust::Vec<rust::String> &items, int height_in_items);

bool BeginMainMenuBar();
void EndMainMenuBar();
bool BeginMenuBar();
void EndMenuBar();
bool BeginMenu(rust::String &label, bool enabled);
void EndMenu();
bool MenuItem(rust::String &label, rust::String &shortcut, bool selected,
              bool enabled);

bool BeginTooltip();
void EndTooltip();
void SetTooltip(rust::String &text, ...);

bool BeginItemTooltip();
void SetItemTooltip(rust::String &text, ...);

bool BeginPopup(rust::String &str_id, int flags);
bool BeginPopupModal(rust::String &name, bool &p_open, int flags);
void EndPopup();

void OpenPopup(rust::String &str_id, int flags);
void OpenPopupOnItemClick(rust::String &str_id, int mouse_button);
void CloseCurrentPopup();

bool BeginPopupContextItem(rust::String &str_id, int mouse_button);
bool BeginPopupContextWindow(rust::String &str_id, int mouse_button,
                             bool also_over_items);
bool BeginPopupContextVoid(rust::String &str_id, int mouse_button);

bool IsPopupOpen(rust::String &str_id, int flags);
}  // namespace rimgui