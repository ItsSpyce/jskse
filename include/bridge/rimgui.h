#pragma once

#include <bridge/rimgui.rs.h>

#define ARRAY_FLOAT(_SIZE) std::array<float, _SIZE>
#define ARRAY_INT(_SIZE) std::array<int, _SIZE>

namespace rimgui {

/// Custom ImGui Builder functions/structs

// Built-in ImGui functions

bool Begin(const std::string& name, bool* open, int flags);
void End();
bool BeginChild(const std::string& id, const ARRAY_FLOAT(2)& size,
                int child_flags, int window_flags);
void EndChild();
bool IsWindowAppearing();
bool IsWindowCollapsed();
bool IsWindowFocused(int flags);
bool IsWindowHovered(int flags);
ARRAY_FLOAT(2) GetWindowPos();
ARRAY_FLOAT(2) GetWindowSize();
float GetWindowWidth();
float GetWindowHeight();
void SetNextWindowPos(const ARRAY_FLOAT(2)& size, int cond,
                      const ARRAY_FLOAT(2)& pivot);
void SetNextWindowSize(const ARRAY_FLOAT(2)& size, int cond);
void SetNextWindowSizeConstraints(const ARRAY_FLOAT(2)& size_min,
                                  const ARRAY_FLOAT(2)& size_max);
void SetNextWindowContentSize(const ARRAY_FLOAT(2)& size);
void SetNextWindowCollapsed(bool collapsed, int cond);
void SetNextWindowFocus();
void SetNextWindowScroll(const ARRAY_FLOAT(2)& scroll);
void SetNextWindowBgAlpha(float alpha);
void SetWindowPos(const ARRAY_FLOAT(2)& pos, int cond);
void SetWindowSize(const ARRAY_FLOAT(2)& size, int cond);
void SetWindowCollapsed(bool collapsed, int cond);
void SetWindowFocus();
void SetWindowFontScale(float scale);
void SetWindowPosByName(const std::string& name, const ARRAY_FLOAT(2)& pos,
                        int cond);
void SetWindowSizeByName(const std::string& name, const ARRAY_FLOAT(2)& size,
                         int cond);
void SetWindowCollapsedByName(const std::string& name, bool collapsed,
                              int cond);
void SetWindowFocusByName(const std::string& name);

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
void PushStyleColor(int idx, const ARRAY_FLOAT(4)& col);
void PopStyleColor(int count);
void PushStyleVar(int index, const ARRAY_FLOAT(2)& val);
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
ARRAY_FLOAT(2) GetFontTextUvWhitePixel();
uint32_t GetColorU32FromStyle(int idx, float alpha_mul);
uint32_t GetColorU32(const ARRAY_FLOAT(4)& col);
ARRAY_FLOAT(4) GetStyleColorVec4(int idx);

ARRAY_FLOAT(2) GetCursorScreenPos();
void SetCursorScreenPos(const ARRAY_FLOAT(2)& pos);
ARRAY_FLOAT(2) GetContentRegionAvail();
ARRAY_FLOAT(2) GetCursorPos();
float GetCursorPosX();
float GetCursorPosY();
void SetCursorPos(const ARRAY_FLOAT(2)& pos);
void SetCursorPosX(float x);
void SetCursorPosY(float y);
ARRAY_FLOAT(2) GetCursorStartPos();

void Separator();
void SameLine(float offset_from_start_x, float spacing);
void NewLine();
void Spacing();
void Dummy(const ARRAY_FLOAT(2)& size);
void Indent(float indent_w);
void Unindent(float indent_w);
void BeginGroup();
void EndGroup();
void AlignTextToFramePadding();
float GetTextLineHeight();
float GetTextLineHeightWithSpacing();
float GetFrameHeight();
float GetFrameHeightWithSpacing();

void PushIDString(const std::string& id);
void PushID(int id);
void PopID();
uint32_t GetIDFromString(const std::string& str);
uint32_t GetID(int id);

void TextUnformatted(const std::string& text);
void Text(const std::string& text, const std::vector<std::string>& args);
void TextColored(const ARRAY_FLOAT(4)& col, const std::string& text,
                 const std::vector<std::string>& args);
void TextDisabled(const std::string& text,
                  const std::vector<std::string>& args);
void TextWrapped(const std::string& text, const std::vector<std::string>& args);
void LabelText(const std::string& label, const std::string& text,
               const std::vector<std::string>& args);
void BulletText(const std::string& text, const std::vector<std::string>& args);
void SeparatorText(const std::string& text);

bool Button(const std::string& label, const ARRAY_FLOAT(2)& size);
bool SmallButton(const std::string& label);
bool InvisibleButton(const std::string& str_id, const ARRAY_FLOAT(2)& size,
                     int flags);
bool ArrowButton(const std::string& str_id, int dir);
bool Checkbox(const std::string& label, bool& v);
bool CheckboxFlags(const std::string& label, int &flags, int flags_value);
bool RadioButtonBool(const std::string& label, bool active);
bool RadioButton(const std::string& label, int& v, int v_button);
void ProgressBar(float fraction, const ARRAY_FLOAT(2)& size_arg,
                 const std::string& overlay);
void Bullet();

void Image(const rust::Vec<uint8_t>& user_texture_bytes,
           const ARRAY_FLOAT(2)& size, const ARRAY_FLOAT(2)& uv0,
           const ARRAY_FLOAT(2)& uv1, const ARRAY_FLOAT(4)& tint_col,
           const ARRAY_FLOAT(4)& border_col);
void ImageFromFile(const std::string& filename, const ARRAY_FLOAT(2)& size,
                   const ARRAY_FLOAT(2)& uv0, const ARRAY_FLOAT(2)& uv1,
                   const ARRAY_FLOAT(4)& tint_col,
                   const ARRAY_FLOAT(4)& border_col);
bool ImageButton(const std::string& str_id, const rust::Vec<uint8_t>& texture,
                 const ARRAY_FLOAT(2)& size, const ARRAY_FLOAT(2)& uv0,
                 const ARRAY_FLOAT(2)& uv1, const ARRAY_FLOAT(4)& bg_col,
                 const ARRAY_FLOAT(4)& tint_col);
bool ImageButtonFromFile(const std::string& str_id, const std::string& filename,
                         const ARRAY_FLOAT(2)& size, const ARRAY_FLOAT(2)& uv0,
                         const ARRAY_FLOAT(2)& uv1, const ARRAY_FLOAT(4)& bg_col,
                         const ARRAY_FLOAT(4)& tint_col);

bool BeginCombo(const std::string& label, const std::string& preview_value,
                int flags);
void EndCombo();
bool Combo(const std::string& label, int& current_item,
           const rust::Vec<rust::String>& items, int height_in_items);

bool DragFloat(const std::string& label, float& v, float v_speed, float v_min,
               float v_max, const std::string& format, int flags);
bool DragFloat2(const std::string& label, ARRAY_FLOAT(2)& v, float v_speed,
                float v_min, float v_max, const std::string& format, int flags);
bool DragFloat3(const std::string& label, ARRAY_FLOAT(3)& v, float v_speed,
                float v_min, float v_max, const std::string& format, int flags);
bool DragFloat4(const std::string& label, ARRAY_FLOAT(4)& v, float v_speed,
                float v_min, float v_max, const std::string& format, int flags);
bool DragFloatRange2(const std::string& label, float& v_current_min,
                     float& v_current_max, float v_speed, float v_min,
                     float v_max, const std::string& format_min,
                     const std::string& format_max, int flags);
bool DragInt(const std::string& label, int& v, float v_speed, int v_min,
             int v_max, const std::string& format, int flags);
bool DragInt2(const std::string& label, ARRAY_INT(2)& v, float v_speed,
              int v_min, int v_max, const std::string& format, int flags);
bool DragInt3(const std::string& label, ARRAY_INT(3)& v, float v_speed,
              int v_min, int v_max, const std::string& format, int flags);
bool DragInt4(const std::string& label, ARRAY_INT(4)& v, float v_speed,
              int v_min, int v_max, const std::string& format, int flags);
bool DragIntRange2(const std::string& label, int& v_current_min,
                   int& v_current_max, float v_speed, int v_min, int v_max,
                   const std::string& format_min, const std::string& format_max,
                   int flags);

bool SliderFloat(const std::string& label, float& v, float v_min, float v_max,
                 const std::string& format, int flags);
bool SliderFloat2(const std::string& label, ARRAY_FLOAT(2)& v, float v_min,
                  float v_max, const std::string& format, int flags);
bool SliderFloat3(const std::string& label, ARRAY_FLOAT(3)& v, float v_min,
                  float v_max, const std::string& format, int flags);
bool SliderFloat4(const std::string& label, ARRAY_FLOAT(4)& v, float v_min,
                  float v_max, const std::string& format, int flags);
bool SliderAngle(const std::string& label, float& v_rad, float v_degrees_min,
                 float v_degrees_max, const std::string& format, int flags);
bool SliderInt(const std::string& label, int& v, int v_min, int v_max,
               const std::string& format, int flags);
bool SliderInt2(const std::string& label, ARRAY_INT(2)& v, int v_min, int v_max,
                const std::string& format, int flags);
bool SliderInt3(const std::string& label, ARRAY_INT(3)& v, int v_min, int v_max,
                const std::string& format, int flags);
bool SliderInt4(const std::string& label, ARRAY_INT(4)& v, int v_min, int v_max,
                const std::string& format, int flags);

bool InputText(const std::string& label, rust::Vec<uint8_t>& buf, int flags);
bool InputTextWithHint(const std::string& label, const std::string& hint,
                       rust::Vec<uint8_t>& buf, int flags);
bool InputFloat(const std::string& label, float& v, float step, float step_fast,
                const std::string& format, int flags);
bool InputFloat2(const std::string& label, ARRAY_FLOAT(2)& v,
                 const std::string& format, int flags);
bool InputFloat3(const std::string& label, ARRAY_FLOAT(3)& v,
                 const std::string& format, int flags);
bool InputFloat4(const std::string& label, ARRAY_FLOAT(4)& v,
                 const std::string& format, int flags);
bool InputInt(const std::string& label, int& v, int step, int step_fast,
              int flags);
bool InputInt2(const std::string& label, ARRAY_INT(2)& v, int flags);
bool InputInt3(const std::string& label, ARRAY_INT(3)& v, int flags);
bool InputInt4(const std::string& label, ARRAY_INT(4)& v, int flags);
bool InputDouble(const std::string& label, double& v, double step,
                 double step_fast, const std::string& format, int flags);

bool ColorEdit3(const std::string& label, ARRAY_FLOAT(3)& col, int flags);
bool ColorEdit4(const std::string& label, ARRAY_FLOAT(4)& col, int flags);
bool ColorPicker3(const std::string& label, ARRAY_FLOAT(3)& col, int flags);
bool ColorPicker4(const std::string& label, ARRAY_FLOAT(4)& col, int flags,
                  const ARRAY_FLOAT(4)& ref_col);
bool ColorButton(const std::string& desc_id, ARRAY_FLOAT(4)& col, int flags,
                 const ARRAY_FLOAT(2)& size);
void SetColorEditOptions(int flags);

bool TreeNode(const std::string& label);
bool TreeNodeWithId(const std::string& str_id, const std::string& fmt);
bool TreeNodeEx(const std::string& label, int flags);
bool TreeNodeExWithId(const std::string& str_id, int flags,
                      const std::string& fmt);
void TreePush(const std::string& str_id);
void TreePop();
float GetTreeNodeToLabelSpacing();
bool CollapsingHeader(const std::string& label, int flags);
void SetNextItemOpen(bool is_open, int cond);
void SetNextItemStorageID(uint32_t id);

bool Selectable(const std::string& label, bool selected, int flags,
                const ARRAY_FLOAT(2)& size);

// TODO: multi-selectable

bool BeginListBox(const std::string& label, const ARRAY_FLOAT(2)& size);
void EndListBox();
bool ListBox(const std::string& label, int& current_item,
             const rust::Vec<rust::String>& items, int height_in_items);

bool BeginMainMenuBar();
void EndMainMenuBar();
bool BeginMenuBar();
void EndMenuBar();
bool BeginMenu(const std::string& label, bool enabled);
void EndMenu();
bool MenuItem(const std::string& label, const std::string& shortcut,
              bool selected, bool enabled);

void BeginTooltip();
void EndTooltip();
void SetTooltip(const std::string& text);

bool BeginItemTooltip();
void SetItemTooltip(const std::string& text);

bool BeginPopup(const std::string& str_id, int flags);
bool BeginPopupModal(const std::string& name, bool& p_open, int flags);
void EndPopup();

void OpenPopup(const std::string& str_id, int flags);
void OpenPopupOnItemClick(const std::string& str_id, int popup_flags);
void CloseCurrentPopup();

bool BeginPopupContextItem(const std::string& str_id, int mouse_button);
bool BeginPopupContextWindow(const std::string& str_id, int mouse_button);
bool BeginPopupContextVoid(const std::string& str_id, int mouse_button);

bool IsPopupOpen(const std::string& str_id, int flags);

bool BeginTable(const std::string& str_id, int column, int flags,
                const ARRAY_FLOAT(2)& outer_size, float inner_width);
void EndTable();
void TableNextRow(int row_flags, float min_row_height);
bool TableNextColumn();
bool TableSetColumnIndex(int column_n);

void TableSetupColumn(const std::string& label, int flags,
                      float init_width_or_weight, uint32_t user_id);
void TableSetupScrollFreeze(int cols, int rows);
void TableHeadersRow();
void TableHeader(const std::string& label);

int TableGetColumnCount();
int TableGetColumnIndex();
int TableGetRowIndex();
rust::String TableGetColumnName(int column_n);
int TableGetColumnFlags(int column_n);
void TableSetColumnEnabled(int column_n, bool enabled);
void TableSetBgColor(int target, uint32_t color, int column_n);

void Columns(int count, const std::string& id, bool border);
void NextColumn();
int GetColumnIndex();
float GetColumnWidth(int column_index);
void SetColumnWidth(int column_index, float width);
float GetColumnOffset(int column_index);
void SetColumnOffset(int column_index, float offset_x);
int GetColumnsCount();

bool BeginTabBar(const std::string& str_id, int flags);
void EndTabBar();
bool BeginTabItem(const std::string& label, bool& p_open, int flags);
void EndTabItem();
bool TabItemButton(const std::string& label, int flags);
void SetTabItemClosed(const std::string& tab_or_docked_window_label);

// TODO: Drag and drop

void BeginDisabled(bool disabled);
void EndDisabled();

void PushClipRect(const ARRAY_FLOAT(2)& clip_rect_min,
                  const ARRAY_FLOAT(2)& clip_rect_max,
                  bool intersect_with_current_clip_rect);
void PopClipRect();

void SetItemDefaultFocus();
void SetKeyboardFocusHere(int offset);

bool IsItemHovered(int flags);
bool IsItemActive();
bool IsItemFocused();
bool IsItemClicked(int mouse_button);
bool IsItemVisible();
bool IsItemEdited();
bool IsItemActivated();
bool IsItemDeactivated();
bool IsItemDeactivatedAfterEdit();
bool IsItemToggledOpen();
bool IsAnyItemHovered();
bool IsAnyItemActive();
bool IsAnyItemFocused();
ARRAY_FLOAT(2) GetItemRectMin();
ARRAY_FLOAT(2) GetItemRectMax();
ARRAY_FLOAT(2) GetItemRectSize();
void SetItemAllowOverlap();

bool IsRectVisible(const ARRAY_FLOAT(2)& rect_min_or_size,
                   const ARRAY_FLOAT(2)& rect_max_or_none);
double GetTime();
int GetFrameCount();
rust::String GetStyleColorName(int idx);
bool BeginChildFrame(uint32_t id, const ARRAY_FLOAT(2)& size, int flags);
void EndChildFrame();

ARRAY_FLOAT(2) CalcTextSize(const std::string& text, const std::string& text_end,
                          bool hide_text_after_double_hash, float wrap_width);

ARRAY_FLOAT(4) ColorConvertU32ToFloat4(uint32_t in);
uint32_t ColorConvertFloat4ToU32(const ARRAY_FLOAT(4)& in);
ARRAY_FLOAT(4) ColorConvertRGBToHSV(const ARRAY_FLOAT(4)& rgb);
ARRAY_FLOAT(4) ColorConvertHSVToRGB(const ARRAY_FLOAT(4)& hsv);

bool IsKeyDown(int user_key_index);
bool IsKeyPressed(int user_key_index, bool repeat);
bool IsKeyReleased(int user_key_index);
int GetKeyPressedAmount(int key_index, float repeat_delay, float rate);
rust::String GetKeyName(int key_index);
void SetNextFrameWantCaptureKeyboard(int value);

bool IsMouseDown(int button);
bool IsMouseClicked(int button, bool repeat);
bool IsMouseReleased(int button);
bool IsMouseDoubleClicked(int button);
int GetMouseClickedCount(int button);
bool IsMouseHoveringRect(const ARRAY_FLOAT(2)& r_min, const ARRAY_FLOAT(2)& r_max,
                         bool clip);
bool IsMousePosValid(const ARRAY_FLOAT(2)& pos);
bool IsAnyMouseDown();
ARRAY_FLOAT(2) GetMousePos();
ARRAY_FLOAT(2) GetMousePosOnOpeningCurrentPopup();
bool IsMouseDragging(int button, float lock_threshold);
ARRAY_FLOAT(2) GetMouseDragDelta(int button, float lock_threshold);
void ResetMouseDragDelta(int button);
int GetMouseCursor();
void SetMouseCursor(int cursor_type);
void SetNextFrameWantCaptureMouse(bool want_capture_mouse);

rust::String GetClipboardText();
void SetClipboardText(const std::string& text);

}  // namespace rimgui