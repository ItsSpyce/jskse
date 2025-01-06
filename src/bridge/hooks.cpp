#include "bridge/hooks.h"

#include "graphics.h"
#include "bridge/strings.rs.h"
#include "hook_builder.h"
#include "jskse_core.h"
#include "util.h"

// Maybe there are more than just the player hook you want to install?
void hooks::install() {
  const auto builder = new HookBuilder();
  builder->add_call<D3DInitHook, 5, 14>({75595, 0x9}, {77226, 0x275},
                                        {0xDC5530, 0x9});
  builder->add_call<DXGIPresentHook, 5, 14>({75461, 0x9}, {77246, 0x9},
                                            {0xDBBDD0, 0x15});
  builder->add_call<ProcessInputQueueHook, 5, 14>({67315, 0x7B}, {68617, 0x7B},
                                                  {0xC519E0, 0x81});
  builder->install();
  PlayerHook::install();
  DialogueHook::install();
}

void hooks::D3DInitHook::thunk() {
  rlog::info("Initializing DirectX..."sv);
  func();
  const auto device = graphics::get_device();
  if (!device) {
    rlog::error("Failed to get device"sv);
  }
}

LRESULT hooks::WndProcHook::thunk(HWND hwnd, UINT msg, WPARAM wparam,
                                  LPARAM lparam) {
  return func(hwnd, msg, wparam, lparam);
}

void hooks::DXGIPresentHook::thunk() {
  rlog::info("Presenting DirectX..."sv);
  func();
  rlog::info("Presented DirectX"sv);
}

void hooks::ProcessInputQueueHook::thunk(
    RE::BSTEventSource<RE::InputEvent*>* dispatcher,
    RE::InputEvent* const* event) {
  rlog::info("Processing input queue..."sv);
  func(dispatcher, event);
  rlog::info("Processed input queue"sv);
}

void hooks::PlayerHook::install() {
  rlog::info("Hooking player so we get inventory changes..."sv);

  REL::Relocation<std::uintptr_t> player_character_vtbl{
      RE::VTABLE_PlayerCharacter[0]};
  add_object_to_container_ =
      player_character_vtbl.write_vfunc(0x5A, item_added);
  pick_up_object_ = player_character_vtbl.write_vfunc(0xCC, item_picked_up);
  remove_item_ = player_character_vtbl.write_vfunc(0x56, item_removed);

  auto& trampoline = SKSE::GetTrampoline();
  REL::Relocation<std::uintptr_t> add_item_functor_hook{
      RELOCATION_ID(55946, 56490)};
  add_item_functor_ = trampoline.write_call<5>(
      add_item_functor_hook.address() + 0x15D, add_item_functor);
  rlog::info("Player hooked.");
}

void hooks::PlayerHook::item_added(RE::Actor* a_this,
                                   RE::TESBoundObject* object,
                                   RE::ExtraDataList* extra_data_list,
                                   int32_t delta,
                                   RE::TESObjectREFR* a_from_refr) {
  // call the original first
  add_object_to_container_(a_this, object, extra_data_list, delta, a_from_refr);
  if (object->IsInventoryObject()) {
    auto item_form = RE::TESForm::LookupByID(object->formID);
    notify_inventory_changed(item_form);
  }
}

void hooks::PlayerHook::item_picked_up(RE::Actor* actor,
                                       RE::TESObjectREFR* object,
                                       uint32_t delta, bool a_arg3,
                                       bool a_play_sound) {
  // call the original first
  pick_up_object_(actor, object, delta, a_arg3, a_play_sound);
  if (object->GetBaseObject()->IsInventoryObject()) {
    auto lookup = object->formID;
    if (lookup == 0) {
      lookup = object->GetBaseObject()->formID;
    }
    auto item_form = RE::TESForm::LookupByID(lookup);
    notify_inventory_changed(item_form);
  }
}

RE::ObjectRefHandle hooks::PlayerHook::item_removed(
    RE::Actor* actor, RE::TESBoundObject* object, std::int32_t delta,
    RE::ITEM_REMOVE_REASON a_reason, RE::ExtraDataList* extra_data_list,
    RE::TESObjectREFR* a_move_to_ref, const RE::NiPoint3* a_drop_loc,
    const RE::NiPoint3* a_rotate) {
  // call the original and snag its return value before we do our thing
  auto retval = remove_item_(actor, object, delta, a_reason, extra_data_list,
                             a_move_to_ref, a_drop_loc, a_rotate);
  if (object->IsInventoryObject()) {
    auto* item_form = RE::TESForm::LookupByID(object->formID);
    notify_inventory_changed(item_form);
  }
  return retval;
}

void hooks::PlayerHook::add_item_functor(RE::TESObjectREFR* a_this,
                                         RE::TESObjectREFR* object,
                                         int32_t delta, bool a4, bool a5) {
  add_item_functor_(a_this, object, delta, a4, a5);
  auto item_form = RE::TESForm::LookupByID(object->GetBaseObject()->formID);
  notify_inventory_changed(item_form);
}

void hooks::PlayerHook::notify_inventory_changed(RE::TESForm* item_form) {
  if (!item_form) {
    return;
  }

  // note that we must make sure the item name is valid utf8 before we log it.
  // if we do not, we crash.
  const auto safename = util::name_as_utf8(item_form);
  rlog::info("inventory change for item '{}'", safename);

  // pour your implementation of something useful to do here!
}

void hooks::DialogueHook::install() {
  //
}