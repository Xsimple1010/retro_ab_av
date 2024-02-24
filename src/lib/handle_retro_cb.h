#include "libretro.h"

typedef void(RETRO_CALLCONV *RustVideoRefreshT)(void *extra_data, const void *data, unsigned width,
                                                unsigned height, size_t pitch);
typedef void(RETRO_CALLCONV *RustAudioSampleT)(void *extra_data, int16_t left, int16_t right);
typedef size_t(RETRO_CALLCONV *RustAudioSampleTBatchT)(void *extra_data, const int16_t *data,
                                                       size_t frames);

//  essa função deve ser enviada para o núcleo
void core_video_refresh(const void *data, unsigned width, //
                        unsigned height, size_t pitch);
void core_audio_sample(int16_t left, int16_t right);
size_t core_audio_sample_batch(const int16_t *data,
                               size_t frames);

// tem que ser chamada antes que qualquer callback ser chamada
void set_video_extra_data(void *extra_data); //
void set_audio_sample_extra_data(void *extra_data);
void set_audio_sample_batch_extra_data(void *extra_data);

void set_rust_video_refresh(RustVideoRefreshT cb);
void set_rust_audio_sample(RustAudioSampleT cb);
void set_rust_audio_sample_batch(RustAudioSampleTBatchT cb);

void de_init_all_callbacks();