#pragma once
#include <REL/Relocation.h>

namespace RE {
inline int64_t AddTopic(RE::MenuTopicManager* a_this, RE::TESTopic* a_topic,
                        int64_t a_3, int64_t a_4) {
  using func_t = decltype(&RE::AddTopic);
  REL::Relocation<func_t> func{REL::ID(35303)};
  return func(a_this, a_topic, a_3, a_4);
}
}  // namespace RE

// Hooking the functions that let us listen for player inventory changes.
// Your plugin might not need to do this, but it's a nice chunky example.
namespace hooks {

static void install();

class PlayerHooks {
 public:
  static void install();

 private:
  static void notify_inventory_changed(RE::TESForm* item_form);
  static void item_added(RE::Actor* a_this, RE::TESBoundObject* a_object,
                  RE::ExtraDataList* a_extra_list, int32_t a_count,
                  RE::TESObjectREFR* a_from_refr);
  static void item_picked_up(RE::Actor* a_this, RE::TESObjectREFR* a_object,
                      uint32_t a_count, bool a_arg3, bool a_play_sound);
  static RE::ObjectRefHandle item_removed(
      RE::Actor* a_this, RE::TESBoundObject* a_item, std::int32_t a_count,
      RE::ITEM_REMOVE_REASON a_reason, RE::ExtraDataList* a_extra_list,
      RE::TESObjectREFR* a_move_to_ref, const RE::NiPoint3* a_drop_loc,
      const RE::NiPoint3* a_rotate);
  static void add_item_functor(RE::TESObjectREFR* a_this, RE::TESObjectREFR* a_object,
                        int32_t a_count, bool a4, bool a5);

  static REL::Relocation<decltype(item_picked_up)> pick_up_object_;
  static REL::Relocation<decltype(item_removed)> remove_item_;
  static REL::Relocation<decltype(item_added)> add_object_to_container_;
  static REL::Relocation<decltype(add_item_functor)> add_item_functor_;
};

class DialogueHooks {
public:
// inspired by
// https://github.com/Scrabx3/Dynamic-Dialogue-Replacer/blob/main/src/Hooks/Hooks.h
static void install();

typedef int64_t(WINAPI* PopulateTopicInfoType)(int64_t, RE::TESTopic*,
                                               RE::TESTopicInfo*,
                                               RE::Character*,
                                               RE::TESTopicInfo::ResponseData*);
struct Response {
  bool keep;
  const char* subtitle;
  const char* voice_path;
};

  class TopicInfo {
  public:
    TopicInfo() = default;
    ~TopicInfo() = default;
    TopicInfo(TopicInfo&&) = delete;
    TopicInfo(const TopicInfo&) = delete;

    _NODISCARD const char *get_subtitle(const size_t index) const {
      if (index >= responses_.size()) {
        return nullptr;
      }
      return responses_[index].subtitle;
    }
    _NODISCARD const char* get_voice_path(const size_t index) const {
      if (index >= responses_.size()) {
        return nullptr;
      }
      return responses_[index].voice_path;
    }

  private:
    RE::FormID topic_info_id_;
    std::vector<Response> responses_;
    uint64_t priority_{0};
    bool random_{false};
    bool cut_{true};
  };

private:
  struct DialogueResponse {
    RE::Character* speaker{nullptr};
    std::shared_ptr<TopicInfo> response{nullptr};
    int32_t response_number{-1};
  };

};
};  // namespace hooks
