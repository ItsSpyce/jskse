#include "bridge/cosave.h"

#include "bridge/cosave.rs.h"

inline const auto UNIQUE_ID = 0x54585844;  // aka "CXXT";
inline const auto RECORD_TAG = _byteswap_ulong('RCRD');
inline const auto FORMAT_VERSION = 1;

void cosave::initialize_cosaves() {
  rlog::info("Registering plugin for SKSE cosaves.");
  auto* cosave = SKSE::GetSerializationInterface();
  cosave->SetUniqueID(UNIQUE_ID);
  cosave->SetSaveCallback(on_game_saved);
  cosave->SetRevertCallback(on_revert);
  cosave->SetLoadCallback(on_game_loaded);
}

void cosave::on_game_saved(SKSE::SerializationInterface* cosave) {
  // The format is an ad-hoc bag of bytes that we interpret
  // as we wish. So we serialize to a bag of bytes on the Rust side.
  constexpr auto version = FORMAT_VERSION;
  rust::Vec<uint8_t> buffer = cosave_data();
  const auto bufsize = static_cast<uint32_t>(buffer.size());
  rlog::debug("cycles serialized into a Vec<u8> of size={}"sv, bufsize);
  if (!cosave->OpenRecord(RECORD_TAG, version)) {
    rlog::error("Unable to open record to write cosave data.");
    return;
  }
  cosave->WriteRecordData(bufsize);
  cosave->WriteRecordData(buffer.data(), bufsize);
}

void cosave::on_revert(SKSE::SerializationInterface*) {
  // This will be plugin-dependent.
}

void cosave::on_game_loaded(SKSE::SerializationInterface* cosave) {
  std::uint32_t type;
  std::uint32_t version;
  std::uint32_t size;
  while (cosave->GetNextRecordInfo(type, version, size)) {
    if (type == RECORD_TAG) {
      rlog::trace("reading cosave data version {}"sv, version);
      uint32_t bufSize;
      std::vector<uint8_t> buffer;
      cosave->ReadRecordData(bufSize);
      buffer.resize(bufSize);
      const auto read = cosave->ReadRecordData(buffer.data(), bufSize);
      buffer.resize(read);
      rlog::trace("read {} bytes from cosave; buffer len is {}"sv, read,
                  buffer.size());
      // pass the buffer over to Rust to deserialize
      cosave_loaded(buffer, version);
    } else {
      rlog::warn("Unknown record type in cosave; type={}", type);
    }
  }
}
