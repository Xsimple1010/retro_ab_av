#include "handle_retro_cb.h"

// video
static RustVideoRefreshT rust_video_refresh_cb;
static void *video_extra_data = NULL;
static void *audio_extra_data = NULL;

void set_video_extra_data(void *data)
{
    video_extra_data = data;
}

void set_audio_extra_data(void *data)
{
    audio_extra_data = data;
}

void set_rust_video_refresh(RustVideoRefreshT cb)
{
    rust_video_refresh_cb = cb;
}

void core_video_refresh(const void *data, unsigned width,
                        unsigned height, size_t pitch)
{
    if (video_extra_data == NULL)
    {
        return;
    }

    rust_video_refresh_cb(video_extra_data, data, width, height, pitch);

    // Free nao é necessário aqui!
    video_extra_data = NULL;
}

// audio_sample
static RustAudioSampleT rust_audio_sample_cb;

void core_audio_sample(int16_t left, int16_t right)
{
    if (audio_extra_data == NULL)
    {
        return;
    }

    rust_audio_sample_cb(audio_extra_data, left, right);

    audio_extra_data = NULL;
}

void set_rust_audio_sample(RustAudioSampleT cb)
{
    rust_audio_sample_cb = cb;
}

// audio_sample_batch
static RustAudioSampleTBatchT rust_audio_sample_batch_cb;

size_t core_audio_sample_batch(const int16_t *data,
                               size_t frames)
{
    if (audio_extra_data == NULL)
    {
        return 0;
    }

    size_t res = rust_audio_sample_batch_cb(audio_extra_data, data, frames);

    audio_extra_data = NULL;

    return res;
}

void set_rust_audio_sample_batch(RustAudioSampleTBatchT cb)
{
    rust_audio_sample_batch_cb = cb;
}

void de_init_all_callbacks()
{
    rust_video_refresh_cb = NULL;
    rust_audio_sample_cb = NULL;
    rust_audio_sample_batch_cb = NULL;
}

void *get_video_extra_data()
{
    return video_extra_data;
}

void *get_audio_extra_data()
{
    return audio_extra_data;
}