#pragma once

template <class T>
concept hook = requires {
  T::thunk;
  T::func;
};
class HookBuilder {
 public:
  enum class WriteKind {
    kCall,
    kBranch,
  };
  struct OffsetDescription {
    int id;
    int offset;
  };

  class GenericHookEntry {
   public:
    virtual void install(SKSE::Trampoline &trampoline) = 0;
    virtual size_t trampoline_size() = 0;
    virtual ~GenericHookEntry() = default;
  };

  template <hook T, std::size_t N, std::size_t TS, WriteKind Kind>
  class HookEntry : public GenericHookEntry {
   public:
    HookEntry(OffsetDescription se, OffsetDescription ae, OffsetDescription vr)
        : se_(se), ae_(ae), vr_(vr) {}

    void install(SKSE::Trampoline &trampoline) override {
      REL::Relocation func{REL::RelocationID(se_.id, ae_.id, vr_.id)};
      if constexpr (Kind == WriteKind::kCall) {
        T::func = trampoline.write_call<N>(
            func.address() + REL::Relocate(se_.offset, ae_.offset, vr_.offset),
            T::thunk);
      } else {
        T::func = trampoline.write_branch<N>(
            func.address() + REL::Relocate(se_.offset, ae_.offset, vr_.offset),
            T::thunk);
      }
    }
    size_t trampoline_size() override { return TS; }

   private:
    OffsetDescription se_;
    OffsetDescription ae_;
    OffsetDescription vr_;
  };
  HookBuilder() = default;
  HookBuilder(HookBuilder &other) {
    hooks_.insert(hooks_.end(), other.hooks_.begin(), other.hooks_.end());
    delete &other;
  }
  HookBuilder(HookBuilder &&other) noexcept {
    hooks_.insert(hooks_.end(), other.hooks_.begin(), other.hooks_.end());
    delete &other;
  }

  ~HookBuilder() {
    for (const auto hook : hooks_) {
      delete hook;
    }
  }

  template <hook T, std::size_t N, std::size_t TS>
  void add_call(OffsetDescription se, OffsetDescription ae,
                OffsetDescription vr) {
    if constexpr (N != 5 && N != 6) {
      static_assert(false && N, "Invalid number of bytes for call");
    }
    const auto entry = new HookEntry<T, N, TS, WriteKind::kCall>(se, ae, vr);
    hooks_.emplace_back(entry);
  }

  template <hook T, std::size_t N, std::size_t TS>
  void add_branch(OffsetDescription se, OffsetDescription ae,
                  OffsetDescription vr) {
    if constexpr (N != 5 && N != 6) {
      static_assert(false && N, "Invalid number of bytes for branch");
    }
    const auto entry = new HookEntry<T, N, TS, WriteKind::kBranch>(se, ae, vr);
    hooks_.emplace_back(entry);
  }

  void add_builder(HookBuilder *builder) {
    hooks_.insert(hooks_.end(), builder->hooks_.begin(), builder->hooks_.end());
    delete builder;
  }

  void install() {
    auto &trampoline = SKSE::GetTrampoline();
    trampoline.create(std::accumulate(hooks_.begin(), hooks_.end(), size_t(0),
                                      [](size_t sum, GenericHookEntry *cls) {
                                        return sum + cls->trampoline_size();
                                      }));
    for (const auto &hook : hooks_) {
      hook->install(trampoline);
      delete hook;
    }
    hooks_.clear();
  }

 private:
  std::vector<GenericHookEntry *> hooks_;
};