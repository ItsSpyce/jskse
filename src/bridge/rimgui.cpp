#include "bridge/rimgui.h"

#include "imgui.h"

static ImVec2 to_imvec2(const RIMGUI_VEC &vec) {
  return ImVec2{vec[0], vec[1]};
}

static RIMGUI_VEC from_imvec2(const ImVec2 vec) {
  return RIMGUI_VEC{vec.x, vec.y};
}

bool rimgui::Begin(rust::String name) {
  return ImGui::Begin(name.c_str());
}

void rimgui::End() { ImGui::End(); }

bool rimgui::BeginChild(rust::String id, const RIMGUI_VEC &size,
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

RIMGUI_VEC rimgui::GetWindowPos() { return from_imvec2(ImGui::GetWindowPos()); }

RIMGUI_VEC rimgui::GetWindowSize() {
  return from_imvec2(ImGui::GetWindowSize());
}

float rimgui::GetWindowWidth() { return ImGui::GetWindowWidth(); }

float rimgui::GetWindowHeight() { return ImGui::GetWindowHeight(); }

void rimgui::SetNextWindowPos(RIMGUI_VEC &size, int cond, RIMGUI_VEC &pivot) {
  ImGui::SetNextWindowPos(to_imvec2(size), cond, to_imvec2(pivot));
}

void rimgui::SetNextWindowSize(RIMGUI_VEC &size, int cond) {
  ImGui::SetNextWindowSize(to_imvec2(size), cond);
}

void rimgui::SetNextWindowSizeConstraints(RIMGUI_VEC &size_min,
                                      RIMGUI_VEC &size_max) {
  ImGui::SetNextWindowSizeConstraints(to_imvec2(size_min), to_imvec2(size_max));
}

void rimgui::SetNextWindowContentSize(RIMGUI_VEC &size) {
  ImGui::SetNextWindowContentSize(to_imvec2(size));
}

void rimgui::SetNextWindowCollapsed(bool collapsed, int cond) {
  ImGui::SetNextWindowCollapsed(collapsed, cond);
}

void rimgui::SetNextWindowFocus() { ImGui::SetNextWindowFocus(); }

void rimgui::SetNextWindowScroll(RIMGUI_VEC &scroll) {
  ImGui::SetNextWindowScroll(to_imvec2(scroll));
}

void rimgui::SetNextWindowBgAlpha(float alpha) {
  ImGui::SetNextWindowBgAlpha(alpha);
}

void rimgui::SetWindowPos(RIMGUI_VEC &pos, int cond) {
  ImGui::SetWindowPos(to_imvec2(pos), cond);
}

void rimgui::SetWindowSize(RIMGUI_VEC &size, int cond) {
  ImGui::SetWindowSize(to_imvec2(size), cond);
}

void rimgui::SetWindowCollapsed(bool collapsed, int cond) {
  ImGui::SetWindowCollapsed(collapsed, cond);
}

void rimgui::SetWindowFocus() { ImGui::SetWindowFocus(); }

void rimgui::SetWindowFontScale(float scale) {
  ImGui::SetWindowFontScale(scale);
}

void rimgui::SetWindowPosByName(rust::String name, RIMGUI_VEC &pos,
                                int cond) {
  ImGui::SetWindowPos(name.c_str(), to_imvec2(pos), cond);
}

void rimgui::SetWindowSizeByName(rust::String name, RIMGUI_VEC &size,
                                 int cond) {
  ImGui::SetWindowSize(name.c_str(), to_imvec2(size), cond);
}

void rimgui::SetWindowCollapsedByName(rust::String name, bool collapsed,
                                      int cond) {
  ImGui::SetWindowCollapsed(name.c_str(), collapsed, cond);
}

void rimgui::SetWindowFocusByName(rust::String name) {
  ImGui::SetWindowFocus(name.c_str());
}
