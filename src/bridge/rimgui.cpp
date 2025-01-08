#include "bridge/rimgui.h"

#include <d3d11.h>
#include <imgui.h>
#include <imgui_internal.h>
#define STB_IMAGE_IMPLEMENTATION
#include "graphics.h"
#include "stb_image.h"

static ImVec2 to_imvec2(const ARRAY_FLOAT(2) & vec) {
  return ImVec2{vec[0], vec[1]};
}

static ImVec2 to_imvec2(ARRAY_FLOAT(2) * vec) {
  return ImVec2{(*vec)[0], (*vec)[1]};
}

static ARRAY_FLOAT(2) from_imvec2(const ImVec2 vec) {
  return ARRAY_FLOAT(2){vec.x, vec.y};
}

static ImVec4 to_imvec4(const ARRAY_FLOAT(4) & vec) {
  return ImVec4{vec[0], vec[1], vec[2], vec[3]};
}

static ImVec4 to_imvec4(ARRAY_FLOAT(4) * vec) {
  return ImVec4{(*vec)[0], (*vec)[1], (*vec)[2], (*vec)[3]};
}

static ARRAY_FLOAT(4) from_imvec4(const ImVec4 vec) {
  return ARRAY_FLOAT(4){vec.x, vec.y, vec.z, vec.w};
}

static bool load_texture_from_bytes(const uint8_t *data, size_t data_size,
                                    ID3D11ShaderResourceView **out_srv,
                                    int *out_width, int *out_height) {
  // Load from memory
  if (data_size == 0) {
    return false;
  }
  // Decode texture from memory
  int width = 0;
  int height = 0;
  int channels = 0;
  unsigned char *pixels =
      stbi_load_from_memory(data, data_size, &width, &height, &channels, 0);
  if (pixels == NULL) {
    return false;
  }
  // Create texture
  D3D11_TEXTURE2D_DESC desc;
  ZeroMemory(&desc, sizeof(desc));
  desc.Width = width;
  desc.Height = height;
  desc.MipLevels = 1;
  desc.ArraySize = 1;
  desc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
  desc.SampleDesc.Count = 1;
  desc.SampleDesc.Quality = 0;
  desc.Usage = D3D11_USAGE_DEFAULT;
  desc.BindFlags = D3D11_BIND_SHADER_RESOURCE;
  desc.CPUAccessFlags = 0;
  desc.MiscFlags = 0;
  D3D11_SUBRESOURCE_DATA subResource;
  subResource.pSysMem = pixels;
  subResource.SysMemPitch = desc.Width * 4;
  subResource.SysMemSlicePitch = 0;
  ID3D11Texture2D *pTexture = NULL;
  auto device = graphics::get_device();
  HRESULT hr = device->CreateTexture2D(&desc, &subResource, &pTexture);
  if (FAILED(hr)) {
    stbi_image_free(pixels);
    return false;
  }
  // Create texture view
  D3D11_SHADER_RESOURCE_VIEW_DESC srvDesc;
  ZeroMemory(&srvDesc, sizeof(srvDesc));
  srvDesc.Format = desc.Format;
  srvDesc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE2D;
  srvDesc.Texture2D.MostDetailedMip = 0;
  srvDesc.Texture2D.MipLevels = 1;
  hr = device->CreateShaderResourceView(pTexture, &srvDesc, out_srv);
  pTexture->Release();
  if (FAILED(hr)) {
    stbi_image_free(pixels);
    return false;
  }
  *out_width = width;
  *out_height = height;
  stbi_image_free(pixels);
  return true;
}

static bool load_texture_from_file(const char *filename,
                                   ID3D11ShaderResourceView **out_srv,
                                   int *out_width, int *out_height) {
  FILE *f = fopen(filename, "rb");
  if (!f) {
    return false;
  }
  fseek(f, 0, SEEK_END);
  auto file_size = static_cast<size_t>(ftell(f));
  if (file_size == -1) {
    fclose(f);
    return false;
  }
  fseek(f, 0, SEEK_SET);
  auto data = IM_ALLOC(file_size);
  fread(data, 1, file_size, f);
  auto result = load_texture_from_bytes(static_cast<uint8_t *>(data), file_size,
                                        out_srv, out_width, out_height);
  IM_FREE(data);
  return result;
}

bool rimgui::Begin(const std::string &name, bool *open, int flags) {
  return ImGui::Begin(name.c_str(), open, flags);
}

void rimgui::End() { ImGui::End(); }

bool rimgui::BeginChild(const std::string &id, const ARRAY_FLOAT(2) & size,
                        const int child_flags, const int window_flags) {
  return ImGui::BeginChild(id.c_str(), to_imvec2(size), child_flags,
                           window_flags);
}

void rimgui::EndChild() { ImGui::EndChild(); }

bool rimgui::IsWindowAppearing() { return ImGui::IsWindowAppearing(); }

bool rimgui::IsWindowCollapsed() { return ImGui::IsWindowCollapsed(); }

bool rimgui::IsWindowFocused(const int flags) {
  return ImGui::IsWindowFocused(flags);
}

bool rimgui::IsWindowHovered(const int flags) {
  return ImGui::IsWindowHovered(flags);
}

ARRAY_FLOAT(2) rimgui::GetWindowPos() {
  return from_imvec2(ImGui::GetWindowPos());
}

ARRAY_FLOAT(2) rimgui::GetWindowSize() {
  return from_imvec2(ImGui::GetWindowSize());
}

float rimgui::GetWindowWidth() { return ImGui::GetWindowWidth(); }

float rimgui::GetWindowHeight() { return ImGui::GetWindowHeight(); }

void rimgui::SetNextWindowPos(const ARRAY_FLOAT(2) & size, int cond,
                              const ARRAY_FLOAT(2) & pivot) {
  ImGui::SetNextWindowPos(to_imvec2(size), cond, to_imvec2(pivot));
}

void rimgui::SetNextWindowSize(const ARRAY_FLOAT(2) & size, int cond) {
  ImGui::SetNextWindowSize(to_imvec2(size), cond);
}

void rimgui::SetNextWindowSizeConstraints(const ARRAY_FLOAT(2) & size_min,
                                          const ARRAY_FLOAT(2) & size_max) {
  ImGui::SetNextWindowSizeConstraints(to_imvec2(size_min), to_imvec2(size_max));
}

void rimgui::SetNextWindowContentSize(const ARRAY_FLOAT(2) & size) {
  ImGui::SetNextWindowContentSize(to_imvec2(size));
}

void rimgui::SetNextWindowCollapsed(bool collapsed, int cond) {
  ImGui::SetNextWindowCollapsed(collapsed, cond);
}

void rimgui::SetNextWindowFocus() { ImGui::SetNextWindowFocus(); }

void rimgui::SetNextWindowScroll(const ARRAY_FLOAT(2) & scroll) {
  ImGui::SetNextWindowScroll(to_imvec2(scroll));
}

void rimgui::SetNextWindowBgAlpha(float alpha) {
  ImGui::SetNextWindowBgAlpha(alpha);
}

void rimgui::SetWindowPos(const ARRAY_FLOAT(2) & pos, int cond) {
  ImGui::SetWindowPos(to_imvec2(pos), cond);
}

void rimgui::SetWindowSize(const ARRAY_FLOAT(2) & size, int cond) {
  ImGui::SetWindowSize(to_imvec2(size), cond);
}

void rimgui::SetWindowCollapsed(bool collapsed, int cond) {
  ImGui::SetWindowCollapsed(collapsed, cond);
}

void rimgui::SetWindowFocus() { ImGui::SetWindowFocus(); }

void rimgui::SetWindowFontScale(float scale) {
  ImGui::SetWindowFontScale(scale);
}

void rimgui::SetWindowPosByName(const std::string &name,
                                const ARRAY_FLOAT(2) & pos, int cond) {
  ImGui::SetWindowPos(name.c_str(), to_imvec2(pos), cond);
}

void rimgui::SetWindowSizeByName(const std::string &name,
                                 const ARRAY_FLOAT(2) & size, int cond) {
  ImGui::SetWindowSize(name.c_str(), to_imvec2(size), cond);
}

void rimgui::SetWindowCollapsedByName(const std::string &name, bool collapsed,
                                      int cond) {
  ImGui::SetWindowCollapsed(name.c_str(), collapsed, cond);
}

void rimgui::SetWindowFocusByName(const std::string &name) {
  ImGui::SetWindowFocus(name.c_str());
}

float rimgui::GetScrollX() { return ImGui::GetScrollX(); }

float rimgui::GetScrollY() { return ImGui::GetScrollY(); }

void rimgui::SetScrollX(float scroll_x) { ImGui::SetScrollX(scroll_x); }

void rimgui::SetScrollY(float scroll_y) { ImGui::SetScrollY(scroll_y); }

float rimgui::GetScrollMaxX() { return ImGui::GetScrollMaxX(); }

float rimgui::GetScrollMaxY() { return ImGui::GetScrollMaxY(); }

void rimgui::SetScrollHereX(float center_x_ratio) {
  ImGui::SetScrollHereX(center_x_ratio);
}

void rimgui::SetScrollHereY(float center_y_ratio) {
  ImGui::SetScrollHereY(center_y_ratio);
}

void rimgui::SetScrollFromPosX(float local_x, float center_x_ratio) {
  ImGui::SetScrollFromPosX(local_x, center_x_ratio);
}

void rimgui::SetScrollFromPosY(float local_y, float center_y_ratio) {
  ImGui::SetScrollFromPosY(local_y, center_y_ratio);
}

void rimgui::PopFont() { ImGui::PopFont(); }

void rimgui::PushStyleColor(int idx, const ARRAY_FLOAT(4) & col) {
  ImGui::PushStyleColor(idx, to_imvec4(col));
}

void rimgui::PopStyleColor(int count) { ImGui::PopStyleColor(count); }

void rimgui::PushStyleVar(int index, const ARRAY_FLOAT(2) & val) {
  if (val.size() == 1) {
    ImGui::PushStyleVar(index, val[0]);
  } else {
    ImGui::PushStyleVar(index, to_imvec2(val));
  }
}

void rimgui::PopStyleVar(int count) { ImGui::PopStyleVar(count); }

void rimgui::PushItemFlag(int option, bool enabled) {
  ImGui::PushItemFlag(option, enabled);
}

void rimgui::PopItemFlag() { ImGui::PopItemFlag(); }

void rimgui::PushItemWidth(float item_width) {
  ImGui::PushItemWidth(item_width);
}

void rimgui::PopItemWidth() { ImGui::PopItemWidth(); }

void rimgui::SetNextItemWidth(float item_width) {
  ImGui::SetNextItemWidth(item_width);
}

float rimgui::CalcItemWidth() { return ImGui::CalcItemWidth(); }

void rimgui::PushTextWrapPos(float wrap_local_pos_x) {
  ImGui::PushTextWrapPos(wrap_local_pos_x);
}

void rimgui::PopTextWrapPos() { ImGui::PopTextWrapPos(); }

float rimgui::GetFontSize() { return ImGui::GetFontSize(); }

ARRAY_FLOAT(2) rimgui::GetFontTextUvWhitePixel() {
  return from_imvec2(ImGui::GetFontTexUvWhitePixel());
}

uint32_t rimgui::GetColorU32(const ARRAY_FLOAT(4) & col) {
  return ImGui::GetColorU32(to_imvec4(col));
}

uint32_t rimgui::GetColorU32FromStyle(int idx, float alpha_mul) {
  return ImGui::GetColorU32(idx, alpha_mul);
}

ARRAY_FLOAT(4) rimgui::GetStyleColorVec4(int idx) {
  return from_imvec4(ImGui::GetStyleColorVec4(idx));
}

ARRAY_FLOAT(2) rimgui::GetCursorScreenPos() {
  return from_imvec2(ImGui::GetCursorScreenPos());
}

void rimgui::SetCursorScreenPos(const ARRAY_FLOAT(2) & pos) {
  ImGui::SetCursorScreenPos(to_imvec2(pos));
}

ARRAY_FLOAT(2) rimgui::GetContentRegionAvail() {
  return from_imvec2(ImGui::GetContentRegionAvail());
}

ARRAY_FLOAT(2) rimgui::GetCursorPos() {
  return from_imvec2(ImGui::GetCursorPos());
}

float rimgui::GetCursorPosX() { return ImGui::GetCursorPosX(); }

float rimgui::GetCursorPosY() { return ImGui::GetCursorPosY(); }

void rimgui::SetCursorPos(const ARRAY_FLOAT(2) & pos) {
  ImGui::SetCursorPos(to_imvec2(pos));
}

void rimgui::SetCursorPosX(float x) { ImGui::SetCursorPosX(x); }

void rimgui::SetCursorPosY(float y) { ImGui::SetCursorPosY(y); }

ARRAY_FLOAT(2) rimgui::GetCursorStartPos() {
  return from_imvec2(ImGui::GetCursorStartPos());
}

void rimgui::Separator() { ImGui::Separator(); }

void rimgui::SameLine(float offset_from_start_x, float spacing) {
  ImGui::SameLine(offset_from_start_x, spacing);
}

void rimgui::NewLine() { ImGui::NewLine(); }

void rimgui::Spacing() { ImGui::Spacing(); }

void rimgui::Dummy(const ARRAY_FLOAT(2) & size) {
  ImGui::Dummy(to_imvec2(size));
}

void rimgui::Indent(float indent_w) { ImGui::Indent(indent_w); }

void rimgui::Unindent(float indent_w) { ImGui::Unindent(indent_w); }

void rimgui::BeginGroup() { ImGui::BeginGroup(); }

void rimgui::EndGroup() { ImGui::EndGroup(); }

void rimgui::AlignTextToFramePadding() { ImGui::AlignTextToFramePadding(); }

float rimgui::GetTextLineHeight() { return ImGui::GetTextLineHeight(); }

float rimgui::GetTextLineHeightWithSpacing() {
  return ImGui::GetTextLineHeightWithSpacing();
}

float rimgui::GetFrameHeight() { return ImGui::GetFrameHeight(); }

float rimgui::GetFrameHeightWithSpacing() {
  return ImGui::GetFrameHeightWithSpacing();
}

void rimgui::PushIDString(const std::string &id) { ImGui::PushID(id.c_str()); }

void rimgui::PushID(int id) { ImGui::PushID(id); }

void rimgui::PopID() { ImGui::PopID(); }

uint32_t rimgui::GetIDFromString(const std::string &str) {
  return ImGui::GetID(str.c_str());
}

uint32_t rimgui::GetID(int id) {
  return ImGui::GetID(reinterpret_cast<void *>(id));
}

void rimgui::TextUnformatted(const std::string &text) {
  ImGui::TextUnformatted(text.c_str());
}

void rimgui::Text(const std::string &text,
                  const std::vector<std::string> &args) {
  const char *fmt = text.c_str();
  ImGui::Text(fmt);
}

void rimgui::TextColored(const ARRAY_FLOAT(4) & col, const std::string &text,
                         const std::vector<std::string> &args) {
  const char *fmt = text.c_str();
  // TODO:
  ImGui::TextColored(to_imvec4(col), fmt);
}

void rimgui::TextDisabled(const std::string &text,
                          const std::vector<std::string> &args) {
  const char *fmt = text.c_str();
  ImGui::TextDisabled(fmt);
}

void rimgui::TextWrapped(const std::string &text,
                         const std::vector<std::string> &args) {
  const char *fmt = text.c_str();
  ImGui::TextWrapped(fmt);
}

void rimgui::LabelText(const std::string &label, const std::string &text,
                       const std::vector<std::string> &args) {
  const char *fmt = text.c_str();
  ImGui::LabelText(label.c_str(), fmt);
}

void rimgui::BulletText(const std::string &text,
                        const std::vector<std::string> &args) {
  const char *fmt = text.c_str();
  ImGui::BulletText(fmt);
}

void rimgui::SeparatorText(const std::string &text) {
  ImGui::Separator();
  ImGui::Text(text.c_str());
}

bool rimgui::Button(const std::string &label, const ARRAY_FLOAT(2) & size) {
  return ImGui::Button(label.c_str(), to_imvec2(size));
}

bool rimgui::SmallButton(const std::string &label) {
  return ImGui::SmallButton(label.c_str());
}

bool rimgui::InvisibleButton(const std::string &str_id,
                             const ARRAY_FLOAT(2) & size, int flags) {
  return ImGui::InvisibleButton(str_id.c_str(), to_imvec2(size), flags);
}

bool rimgui::ArrowButton(const std::string &str_id, int dir) {
  return ImGui::ArrowButton(str_id.c_str(), dir);
}

bool rimgui::Checkbox(const std::string &label, bool *v) {
  return ImGui::Checkbox(label.c_str(), v);
}

bool rimgui::CheckboxFlags(const std::string &label, int *flags,
                           int flags_value) {
  return ImGui::CheckboxFlags(label.c_str(), flags, flags_value);
}

bool rimgui::RadioButtonBool(const std::string &label, bool active) {
  return ImGui::RadioButton(label.c_str(), active);
}

bool rimgui::RadioButton(const std::string &label, int *v, int v_button) {
  return ImGui::RadioButton(label.c_str(), v, v_button);
}

void rimgui::ProgressBar(float fraction, const ARRAY_FLOAT(2) & size_arg,
                         const std::string &overlay) {
  ImGui::ProgressBar(fraction, to_imvec2(size_arg), overlay.c_str());
}

void rimgui::Bullet() { ImGui::Bullet(); }

void rimgui::Image(const rust::Vec<uint8_t> &user_texture_bytes,
                   const ARRAY_FLOAT(2) & size, const ARRAY_FLOAT(2) & uv0,
                   const ARRAY_FLOAT(2) & uv1, const ARRAY_FLOAT(4) & tint_col,
                   const ARRAY_FLOAT(4) & border_col) {
  ID3D11ShaderResourceView *tex = NULL;
  int height = 0, width = 0;
  if (load_texture_from_bytes(user_texture_bytes.data(),
                              user_texture_bytes.size(), &tex, &width,
                              &height)) {
    ImGui::Image(tex, to_imvec2(size), to_imvec2(uv0), to_imvec2(uv1),
                 to_imvec4(tint_col), to_imvec4(border_col));
  }
}

void rimgui::ImageFromFile(const std::string &filename,
                           const ARRAY_FLOAT(2) & size,
                           const ARRAY_FLOAT(2) & uv0,
                           const ARRAY_FLOAT(2) & uv1,
                           const ARRAY_FLOAT(4) & tint_col,
                           const ARRAY_FLOAT(4) & border_col) {
  ID3D11ShaderResourceView *tex = NULL;
  int height = 0, width = 0;
  if (load_texture_from_file(filename.c_str(), &tex, &width, &height)) {
    ImGui::Image(tex, to_imvec2(size), to_imvec2(uv0), to_imvec2(uv1),
                 to_imvec4(tint_col), to_imvec4(border_col));
  }
}

bool rimgui::ImageButton(const std::string &str_id,
                         const rust::Vec<uint8_t> &texture,
                         const ARRAY_FLOAT(2) & size,
                         const ARRAY_FLOAT(2) & uv0, const ARRAY_FLOAT(2) & uv1,
                         const ARRAY_FLOAT(4) & bg_col,
                         const ARRAY_FLOAT(4) & tint_col) {
  ID3D11ShaderResourceView *tex = NULL;
  int height = 0, width = 0;
  if (load_texture_from_bytes(texture.data(), texture.size(), &tex, &width,
                              &height)) {
    return ImGui::ImageButton(str_id.c_str(), tex, to_imvec2(size),
                              to_imvec2(uv0), to_imvec2(uv1), to_imvec4(bg_col),
                              to_imvec4(tint_col));
  }
  return false;
}

bool rimgui::ImageButtonFromFile(const std::string &str_id,
                                 const std::string &filename,
                                 const ARRAY_FLOAT(2) & size,
                                 const ARRAY_FLOAT(2) & uv0,
                                 const ARRAY_FLOAT(2) & uv1,
                                 const ARRAY_FLOAT(4) & bg_col,
                                 const ARRAY_FLOAT(4) & tint_col) {
  ID3D11ShaderResourceView *tex = NULL;
  int height = 0, width = 0;
  if (load_texture_from_file(filename.c_str(), &tex, &width, &height)) {
    return ImGui::ImageButton(str_id.c_str(), tex, to_imvec2(size),
                              to_imvec2(uv0), to_imvec2(uv1), to_imvec4(bg_col),
                              to_imvec4(tint_col));
  }
  return false;
}

bool rimgui::BeginCombo(const std::string &label,
                        const std::string &preview_value, int flags) {
  return ImGui::BeginCombo(label.c_str(), preview_value.c_str(), flags);
}

void rimgui::EndCombo() { ImGui::EndCombo(); }

bool rimgui::Combo(const std::string &label, int *current_item,
                   const rust::Vec<rust::String> &items, int height_in_items) {
  std::vector<const char *> c_items;
  for (auto item : items) {
    c_items.push_back(item.c_str());
  }
  return ImGui::Combo(label.c_str(), current_item, c_items.data(), items.size(),
                      height_in_items);
}

bool rimgui::DragFloat(const std::string &label, float *v, float v_speed,
                       float v_min, float v_max, const std::string &format,
                       int flags) {
  return ImGui::DragFloat(label.c_str(), v, v_speed, v_min, v_max,
                          format.c_str(), flags);
}

bool rimgui::DragFloat2(const std::string &label, ARRAY_FLOAT(2) * v,
                        float v_speed, float v_min, float v_max,
                        const std::string &format, int flags) {
  return ImGui::DragFloat2(label.c_str(), v->data(), v_speed, v_min, v_max,
                           format.c_str(), flags);
}

bool rimgui::DragFloat3(const std::string &label, ARRAY_FLOAT(3) * v,
                        float v_speed, float v_min, float v_max,
                        const std::string &format, int flags) {
  return ImGui::DragFloat3(label.c_str(), v->data(), v_speed, v_min, v_max,
                           format.c_str(), flags);
}

bool rimgui::DragFloat4(const std::string &label, ARRAY_FLOAT(4) * v,
                        float v_speed, float v_min, float v_max,
                        const std::string &format, int flags) {
  return ImGui::DragFloat4(label.c_str(), v->data(), v_speed, v_min, v_max,
                           format.c_str(), flags);
}

bool rimgui::DragFloatRange2(const std::string &label, float *v_current_min,
                             float *v_current_max, float v_speed, float v_min,
                             float v_max, const std::string &format_min,
                             const std::string &format_max, int flags) {
  return ImGui::DragFloatRange2(label.c_str(), v_current_min, v_current_max,
                                v_speed, v_min, v_max, format_min.c_str(),
                                format_max.c_str(), flags);
}

bool rimgui::DragInt(const std::string &label, int *v, float v_speed, int v_min,
                     int v_max, const std::string &format, int flags) {
  return ImGui::DragInt(label.c_str(), v, v_speed, v_min, v_max, format.c_str(),
                        flags);
}

bool rimgui::DragInt2(const std::string &label, ARRAY_INT(2) * v, float v_speed,
                      int v_min, int v_max, const std::string &format,
                      int flags) {
  return ImGui::DragInt2(label.c_str(), v->data(), v_speed, v_min, v_max,
                         format.c_str(), flags);
}

bool rimgui::DragInt3(const std::string &label, ARRAY_INT(3) * v, float v_speed,
                      int v_min, int v_max, const std::string &format,
                      int flags) {
  return ImGui::DragInt3(label.c_str(), v->data(), v_speed, v_min, v_max,
                         format.c_str(), flags);
}

bool rimgui::DragInt4(const std::string &label, ARRAY_INT(4) * v, float v_speed,
                      int v_min, int v_max, const std::string &format,
                      int flags) {
  return ImGui::DragInt4(label.c_str(), v->data(), v_speed, v_min, v_max,
                         format.c_str(), flags);
}

bool rimgui::DragIntRange2(const std::string &label, int *v_current_min,
                           int *v_current_max, float v_speed, int v_min,
                           int v_max, const std::string &format_min,
                           const std::string &format_max, int flags) {
  return ImGui::DragIntRange2(label.c_str(), v_current_min, v_current_max,
                              v_speed, v_min, v_max, format_min.c_str(),
                              format_max.c_str(), flags);
}

bool rimgui::SliderFloat(const std::string &label, float *v, float v_min,
                         float v_max, const std::string &format, int flags) {
  return ImGui::SliderFloat(label.c_str(), v, v_min, v_max, format.c_str(),
                            flags);
}

bool rimgui::SliderFloat2(const std::string &label, ARRAY_FLOAT(2) * v,
                          float v_min, float v_max, const std::string &format,
                          int flags) {
  return ImGui::SliderFloat2(label.c_str(), v->data(), v_min, v_max,
                             format.c_str(), flags);
}

bool rimgui::SliderFloat3(const std::string &label, ARRAY_FLOAT(3) * v,
                          float v_min, float v_max, const std::string &format,
                          int flags) {
  return ImGui::SliderFloat3(label.c_str(), v->data(), v_min, v_max,
                             format.c_str(), flags);
}

bool rimgui::SliderFloat4(const std::string &label, ARRAY_FLOAT(4) * v,
                          float v_min, float v_max, const std::string &format,
                          int flags) {
  return ImGui::SliderFloat4(label.c_str(), v->data(), v_min, v_max,
                             format.c_str(), flags);
}

bool rimgui::SliderAngle(const std::string &label, float *v_rad,
                         float v_degrees_min, float v_degrees_max,
                         const std::string &format, int flags) {
  return ImGui::SliderAngle(label.c_str(), v_rad, v_degrees_min, v_degrees_max,
                            format.c_str(), flags);
}

bool rimgui::SliderInt(const std::string &label, int *v, int v_min, int v_max,
                       const std::string &format, int flags) {
  return ImGui::SliderInt(label.c_str(), v, v_min, v_max, format.c_str(),
                          flags);
}

bool rimgui::SliderInt2(const std::string &label, ARRAY_INT(2) * v, int v_min,
                        int v_max, const std::string &format, int flags) {
  return ImGui::SliderInt2(label.c_str(), v->data(), v_min, v_max,
                           format.c_str(), flags);
}

bool rimgui::SliderInt3(const std::string &label, ARRAY_INT(3) * v, int v_min,
                        int v_max, const std::string &format, int flags) {
  return ImGui::SliderInt3(label.c_str(), v->data(), v_min, v_max,
                           format.c_str(), flags);
}

bool rimgui::SliderInt4(const std::string &label, ARRAY_INT(4) * v, int v_min,
                        int v_max, const std::string &format, int flags) {
  return ImGui::SliderInt4(label.c_str(), v->data(), v_min, v_max,
                           format.c_str(), flags);
}

bool rimgui::InputText(const std::string &label, rust::Vec<uint8_t> &buf,
                       int flags) {
  return ImGui::InputText(label.c_str(), reinterpret_cast<char *>(buf.data()),
                          buf.size(), flags);
}

bool rimgui::InputTextWithHint(const std::string &label,
                               const std::string &hint, rust::Vec<uint8_t> &buf,
                               int flags) {
  return ImGui::InputTextWithHint(label.c_str(), hint.c_str(),
                                  reinterpret_cast<char *>(buf.data()),
                                  buf.size(), flags);
}

bool rimgui::InputFloat(const std::string &label, float *v, float step,
                        float step_fast, const std::string &format, int flags) {
  return ImGui::InputFloat(label.c_str(), v, step, step_fast, format.c_str(),
                           flags);
}

bool rimgui::InputFloat2(const std::string &label, ARRAY_FLOAT(2) * v,
                         const std::string &format, int flags) {
  return ImGui::InputFloat2(label.c_str(), v->data(), format.c_str(), flags);
}

bool rimgui::InputFloat3(const std::string &label, ARRAY_FLOAT(3) * v,
                         const std::string &format, int flags) {
  return ImGui::InputFloat3(label.c_str(), v->data(), format.c_str(), flags);
}

bool rimgui::InputFloat4(const std::string &label, ARRAY_FLOAT(4) * v,
                         const std::string &format, int flags) {
  return ImGui::InputFloat4(label.c_str(), v->data(), format.c_str(), flags);
}

bool rimgui::InputInt(const std::string &label, int *v, int step, int step_fast,
                      int flags) {
  return ImGui::InputInt(label.c_str(), v, step, step_fast, flags);
}

bool rimgui::InputInt2(const std::string &label, ARRAY_INT(2) * v, int flags) {
  return ImGui::InputInt2(label.c_str(), v->data(), flags);
}

bool rimgui::InputInt3(const std::string &label, ARRAY_INT(3) * v, int flags) {
  return ImGui::InputInt3(label.c_str(), v->data(), flags);
}

bool rimgui::InputInt4(const std::string &label, ARRAY_INT(4) * v, int flags) {
  return ImGui::InputInt4(label.c_str(), v->data(), flags);
}

bool rimgui::InputDouble(const std::string &label, double *v, double step,
                         double step_fast, const std::string &format,
                         int flags) {
  return ImGui::InputDouble(label.c_str(), v, step, step_fast, format.c_str(),
                            flags);
}

bool rimgui::ColorEdit3(const std::string &label, ARRAY_FLOAT(3) * col,
                        int flags) {
  return ImGui::ColorEdit3(label.c_str(), col->data(), flags);
}

bool rimgui::ColorEdit4(const std::string &label, ARRAY_FLOAT(4) * col,
                        int flags) {
  return ImGui::ColorEdit4(label.c_str(), col->data(), flags);
}

bool rimgui::ColorPicker3(const std::string &label, ARRAY_FLOAT(3) * col,
                          int flags) {
  return ImGui::ColorPicker3(label.c_str(), col->data(), flags);
}

bool rimgui::ColorPicker4(const std::string &label, ARRAY_FLOAT(4) * col,
                          int flags, const ARRAY_FLOAT(4) & ref_col) {
  return ImGui::ColorPicker4(label.c_str(), col->data(), flags, ref_col.data());
}

bool rimgui::ColorButton(const std::string &desc_id, ARRAY_FLOAT(4) * col,
                         int flags, const ARRAY_FLOAT(2) & size) {
  return ImGui::ColorButton(desc_id.c_str(), to_imvec4(col), flags,
                            to_imvec2(size));
}

void rimgui::Columns(int count, const std::string &id, bool border) {
  ImGui::Columns(count, id.c_str(), border);
}

void rimgui::NextColumn() { ImGui::NextColumn(); }

int rimgui::GetColumnIndex() { return ImGui::GetColumnIndex(); }

float rimgui::GetColumnWidth(int column_index) {
  return ImGui::GetColumnWidth(column_index);
}

void rimgui::SetColumnWidth(int column_index, float width) {
  ImGui::SetColumnWidth(column_index, width);
}

float rimgui::GetColumnOffset(int column_index) {
  return ImGui::GetColumnOffset(column_index);
}

void rimgui::SetColumnOffset(int column_index, float offset_x) {
  ImGui::SetColumnOffset(column_index, offset_x);
}

int rimgui::GetColumnsCount() { return ImGui::GetColumnsCount(); }

bool rimgui::BeginTabBar(const std::string &str_id, int flags) {
  return ImGui::BeginTabBar(str_id.c_str(), flags);
}

void rimgui::EndTabBar() { ImGui::EndTabBar(); }

bool rimgui::BeginTabItem(const std::string &label, bool *p_open, int flags) {
  return ImGui::BeginTabItem(label.c_str(), p_open, flags);
}

void rimgui::EndTabItem() { ImGui::EndTabItem(); }

bool rimgui::TabItemButton(const std::string &label, int flags) {
  return ImGui::TabItemButton(label.c_str(), flags);
}

void rimgui::SetTabItemClosed(const std::string &tab_or_docked_window_label) {
  ImGui::SetTabItemClosed(tab_or_docked_window_label.c_str());
}

void rimgui::BeginDisabled(bool disabled) { ImGui::BeginDisabled(disabled); }

void rimgui::EndDisabled() { ImGui::EndDisabled(); }

void rimgui::PushClipRect(const ARRAY_FLOAT(2) & clip_rect_min,
                          const ARRAY_FLOAT(2) & clip_rect_max,
                          bool intersect_with_current_clip_rect) {
  ImGui::PushClipRect(to_imvec2(clip_rect_min), to_imvec2(clip_rect_max),
                      intersect_with_current_clip_rect);
}

void rimgui::PopClipRect() { ImGui::PopClipRect(); }

void rimgui::SetItemDefaultFocus() { ImGui::SetItemDefaultFocus(); }

void rimgui::SetKeyboardFocusHere(int offset) {
  ImGui::SetKeyboardFocusHere(offset);
}

bool rimgui::IsItemHovered(int flags) { return ImGui::IsItemHovered(flags); }

bool rimgui::IsItemActive() { return ImGui::IsItemActive(); }

bool rimgui::IsItemFocused() { return ImGui::IsItemFocused(); }

bool rimgui::IsItemClicked(int mouse_button) {
  return ImGui::IsItemClicked(mouse_button);
}

bool rimgui::IsItemVisible() { return ImGui::IsItemVisible(); }

bool rimgui::IsItemEdited() { return ImGui::IsItemEdited(); }

bool rimgui::IsItemActivated() { return ImGui::IsItemActivated(); }

bool rimgui::IsItemDeactivated() { return ImGui::IsItemDeactivated(); }

bool rimgui::IsItemDeactivatedAfterEdit() {
  return ImGui::IsItemDeactivatedAfterEdit();
}

bool rimgui::IsItemToggledOpen() { return ImGui::IsItemToggledOpen(); }

bool rimgui::IsAnyItemHovered() { return ImGui::IsAnyItemHovered(); }

bool rimgui::IsAnyItemActive() { return ImGui::IsAnyItemActive(); }

bool rimgui::IsAnyItemFocused() { return ImGui::IsAnyItemFocused(); }

ARRAY_FLOAT(2) rimgui::GetItemRectMin() {
  return from_imvec2(ImGui::GetItemRectMin());
}

ARRAY_FLOAT(2) rimgui::GetItemRectMax() {
  return from_imvec2(ImGui::GetItemRectMax());
}

ARRAY_FLOAT(2) rimgui::GetItemRectSize() {
  return from_imvec2(ImGui::GetItemRectSize());
}

void rimgui::SetItemAllowOverlap() { ImGui::SetItemAllowOverlap(); }

bool rimgui::IsRectVisible(const ARRAY_FLOAT(2) & rect_min_or_size,
                           const ARRAY_FLOAT(2) & rect_max_or_none) {
  return ImGui::IsRectVisible(to_imvec2(rect_min_or_size),
                              to_imvec2(rect_max_or_none));
}

double rimgui::GetTime() { return ImGui::GetTime(); }

int rimgui::GetFrameCount() { return ImGui::GetFrameCount(); }

rust::String rimgui::GetStyleColorName(int idx) {
  return rust::String{ImGui::GetStyleColorName(idx)};
}

bool rimgui::BeginChildFrame(uint32_t id, const ARRAY_FLOAT(2) & size,
                             int flags) {
  return ImGui::BeginChildFrame(id, to_imvec2(size), flags);
}

void rimgui::EndChildFrame() { ImGui::EndChildFrame(); }

ARRAY_FLOAT(2)
rimgui::CalcTextSize(const std::string &text, const std::string &text_end,
                     bool hide_text_after_double_hash, float wrap_width) {
  return from_imvec2(ImGui::CalcTextSize(
      text.c_str(), text_end.c_str(), hide_text_after_double_hash, wrap_width));
}

ARRAY_FLOAT(4) rimgui::ColorConvertU32ToFloat4(uint32_t in) {
  return from_imvec4(ImGui::ColorConvertU32ToFloat4(in));
}

uint32_t rimgui::ColorConvertFloat4ToU32(const ARRAY_FLOAT(4) & in) {
  return ImGui::ColorConvertFloat4ToU32(to_imvec4(in));
}

ARRAY_FLOAT(4) rimgui::ColorConvertRGBToHSV(const ARRAY_FLOAT(4) & rgb) {
  float h, s, v;
  ImGui::ColorConvertRGBtoHSV(rgb[0], rgb[1], rgb[2], h, s, v);
  return ARRAY_FLOAT(4){h, s, v, 1.0f};
}

ARRAY_FLOAT(4) rimgui::ColorConvertHSVToRGB(const ARRAY_FLOAT(4) & hsv) {
  float r, g, b;
  ImGui::ColorConvertHSVtoRGB(hsv[0], hsv[1], hsv[2], r, g, b);
  return ARRAY_FLOAT(4){r, g, b, 1.0f};
}

bool rimgui::IsKeyDown(int user_key_index) {
  return ImGui::IsKeyDown(static_cast<ImGuiKey>(user_key_index));
}

bool rimgui::IsKeyPressed(int user_key_index, bool repeat) {
  return ImGui::IsKeyPressed(static_cast<ImGuiKey>(user_key_index), repeat);
}

bool rimgui::IsKeyReleased(int user_key_index) {
  return ImGui::IsKeyReleased(static_cast<ImGuiKey>(user_key_index));
}

int rimgui::GetKeyPressedAmount(int key_index, float repeat_delay, float rate) {
  return ImGui::GetKeyPressedAmount(static_cast<ImGuiKey>(key_index),
                                    repeat_delay, rate);
}

rust::String rimgui::GetKeyName(int key_index) {
  return ImGui::GetKeyName(static_cast<ImGuiKey>(key_index));
}

void rimgui::SetNextFrameWantCaptureKeyboard(int value) {
  ImGui::SetNextFrameWantCaptureKeyboard(value);
}

bool rimgui::IsMouseDown(int button) { return ImGui::IsMouseDown(button); }

bool rimgui::IsMouseClicked(int button, bool repeat) {
  return ImGui::IsMouseClicked(button, repeat);
}

bool rimgui::IsMouseReleased(int button) {
  return ImGui::IsMouseReleased(button);
}

bool rimgui::IsMouseDoubleClicked(int button) {
  return ImGui::IsMouseDoubleClicked(button);
}

int rimgui::GetMouseClickedCount(int button) {
  return ImGui::GetMouseClickedCount(button);
}

bool rimgui::IsMouseHoveringRect(const ARRAY_FLOAT(2) & r_min,
                                 const ARRAY_FLOAT(2) & r_max, bool clip) {
  return ImGui::IsMouseHoveringRect(to_imvec2(r_min), to_imvec2(r_max), clip);
}

bool rimgui::TreeNode(const std::string &label) {
  return ImGui::TreeNode(label.c_str());
}

bool rimgui::TreeNodeEx(const std::string &label, int flags) {
  return ImGui::TreeNodeEx(label.c_str(), flags);
}

void rimgui::TreePush(const std::string &str_id) {
  ImGui::TreePush(str_id.c_str());
}

void rimgui::TreePop() { ImGui::TreePop(); }

float rimgui::GetTreeNodeToLabelSpacing() {
  return ImGui::GetTreeNodeToLabelSpacing();
}

bool rimgui::CollapsingHeader(const std::string &label, int flags) {
  return ImGui::CollapsingHeader(label.c_str(), flags);
}

bool rimgui::CollapsingHeaderCtrl(const std::string &label, bool *p_open,
                                  int flags) {
  return ImGui::CollapsingHeader(label.c_str(), p_open, flags);
}

void rimgui::SetNextItemOpen(bool is_open, int cond) {
  ImGui::SetNextItemOpen(is_open, cond);
}

bool rimgui::Selectable(const std::string &label, bool *selected, int flags,
                        const ARRAY_FLOAT(2) & size) {
  return ImGui::Selectable(label.c_str(), selected, flags, to_imvec2(size));
}

bool rimgui::ListBox(const std::string &label, int *current_item,
                     const rust::Vec<rust::String> &items,
                     int height_in_items) {
  std::vector<const char *> c_items;
  for (auto item : items) {
    c_items.push_back(item.c_str());
  }
  return ImGui::ListBox(label.c_str(), current_item, c_items.data(),
                        c_items.size(), height_in_items);
}

bool rimgui::BeginListBox(const std::string &label,
                          const ARRAY_FLOAT(2) & size) {
  return ImGui::BeginListBox(label.c_str(), to_imvec2(size));
}

void rimgui::EndListBox() { ImGui::EndListBox(); }

bool rimgui::BeginMenuBar() { return ImGui::BeginMenuBar(); }

void rimgui::EndMenuBar() { ImGui::EndMenuBar(); }

bool rimgui::BeginMainMenuBar() { return ImGui::BeginMainMenuBar(); }

void rimgui::EndMainMenuBar() { ImGui::EndMainMenuBar(); }

bool rimgui::BeginMenu(const std::string &label, bool enabled) {
  return ImGui::BeginMenu(label.c_str(), enabled);
}

void rimgui::EndMenu() { ImGui::EndMenu(); }

bool rimgui::MenuItem(const std::string &label, const std::string &shortcut,
                      bool selected, bool enabled) {
  return ImGui::MenuItem(label.c_str(), shortcut.c_str(), selected, enabled);
}

void rimgui::BeginTooltip() { ImGui::BeginTooltip(); }

void rimgui::EndTooltip() { ImGui::EndTooltip(); }

void rimgui::SetTooltip(const std::string &text) {
  const char *fmt = text.c_str();
  ImGui::SetTooltip(fmt);
}

bool rimgui::BeginPopup(const std::string &str_id, int flags) {
  return ImGui::BeginPopup(str_id.c_str(), flags);
}

bool rimgui::BeginPopupModal(const std::string &name, bool *p_open, int flags) {
  return ImGui::BeginPopupModal(name.c_str(), p_open, flags);
}

void rimgui::EndPopup() { ImGui::EndPopup(); }

void rimgui::OpenPopup(const std::string &str_id, int popup_flags) {
  ImGui::OpenPopup(str_id.c_str(), popup_flags);
}

void rimgui::OpenPopupOnItemClick(const std::string &str_id, int popup_flags) {
  ImGui::OpenPopupOnItemClick(str_id.c_str(), popup_flags);
}

void rimgui::CloseCurrentPopup() { ImGui::CloseCurrentPopup(); }

bool rimgui::BeginPopupContextItem(const std::string &str_id,
                                   int mouse_button) {
  return ImGui::BeginPopupContextItem(str_id.c_str(), mouse_button);
}

bool rimgui::BeginPopupContextWindow(const std::string &str_id,
                                     int mouse_button) {
  return ImGui::BeginPopupContextWindow(str_id.c_str(), mouse_button);
}

bool rimgui::BeginPopupContextVoid(const std::string &str_id,
                                   int mouse_button) {
  return ImGui::BeginPopupContextVoid(str_id.c_str(), mouse_button);
}

bool rimgui::IsPopupOpen(const std::string &str_id, int flags) {
  return ImGui::IsPopupOpen(str_id.c_str(), flags);
}

bool rimgui::BeginTable(const std::string &str_id, int columns_count, int flags,
                        const ARRAY_FLOAT(2) & outer_size, float inner_width) {
  return ImGui::BeginTable(str_id.c_str(), columns_count, flags,
                           to_imvec2(outer_size), inner_width);
}

void rimgui::EndTable() { ImGui::EndTable(); }

void rimgui::TableNextRow(int row_flags, float min_row_height) {
  ImGui::TableNextRow(row_flags, min_row_height);
}

bool rimgui::TableNextColumn() { return ImGui::TableNextColumn(); }

int rimgui::TableGetColumnIndex() { return ImGui::TableGetColumnIndex(); }

bool rimgui::TableSetColumnIndex(int column_n) {
  return ImGui::TableSetColumnIndex(column_n);
}

void rimgui::TableSetupColumn(const std::string &label, int flags,
                              float init_width_or_weight, uint32_t user_id) {
  ImGui::TableSetupColumn(label.c_str(), flags, init_width_or_weight, user_id);
}

void rimgui::TableSetupScrollFreeze(int cols, int rows) {
  ImGui::TableSetupScrollFreeze(cols, rows);
}

void rimgui::TableHeadersRow() { ImGui::TableHeadersRow(); }

void rimgui::TableHeader(const std::string &label) {
  ImGui::TableHeader(label.c_str());
}

int rimgui::TableGetColumnCount() { return ImGui::TableGetColumnCount(); }

int rimgui::TableGetRowIndex() { return ImGui::TableGetRowIndex(); }

rust::String rimgui::TableGetColumnName(int column_n) {
  return ImGui::TableGetColumnName(column_n);
}

int rimgui::TableGetColumnFlags(int column_n) {
  return ImGui::TableGetColumnFlags(column_n);
}

void rimgui::TableSetBgColor(int target, uint32_t color, int column_n) {
  ImGui::TableSetBgColor(target, color, column_n);
}

void rimgui::TableSetColumnEnabled(int column_n, bool enabled) {
  ImGui::TableSetColumnEnabled(column_n, enabled);
}

bool rimgui::BeginItemTooltip() { return false; }
