<script lang='ts'>
  import type { Content } from '$lib/types/item';
  import * as Tabs from '$lib/components/ui/tabs/index.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { ImageOffIcon } from 'lucide-svelte';

  let {
    content = $bindable(),
    paused = $bindable(),
    currentTime = $bindable(),
    volume = $bindable(),
    duration = $bindable(),
    disable_audio = $bindable(),
    onended,
  }: {
    content: Content;
    paused: boolean;
    currentTime: number;
    volume: number;
    duration: number;
    disable_audio: boolean;
    onended: () => void;
  } = $props();

  const is_video = $derived(content.kind === 'Video');

  let default_value: string = $state('thumbnail');

  $effect(() => {
    if (is_video) {
      disable_audio = true;
      default_value = 'video';
    }
    else {
      default_value = 'thumbnail';
    }
  });

</script>

<Tabs.Root value={default_value} class='container mx-auto text-center h-full'>
  <Tabs.List>
    <Tabs.Trigger value='thumbnail'>Thumbnail</Tabs.Trigger>
    {#if is_video}
      <Tabs.Trigger value='video'>Video</Tabs.Trigger>
    {:else}
      <Tooltip.Provider>
        <Tooltip.Root delayDuration={0}>
          <Tooltip.Trigger class='cursor-not-allowed'>
            <Tabs.Trigger value='video' disabled>Video</Tabs.Trigger>
          </Tooltip.Trigger>
          <Tooltip.Content>
            <p>This content is not Video</p>
          </Tooltip.Content>
        </Tooltip.Root>
      </Tooltip.Provider>
    {/if}
  </Tabs.List>
  <Tabs.Content value='thumbnail' class='pt-4 h-full'>
    {#if content.thumbnail_path}
      <img
        src={convertFileSrc(content.thumbnail_path)}
        alt='Thumbnail'
        class='w-full h-auto'
      />
    {:else}
      <ImageOffIcon class='w-full h-full' />
    {/if}
  </Tabs.Content>
  <Tabs.Content value='video' class='pt-4 h-full'>
    <video bind:volume bind:currentTime bind:paused bind:duration onended={onended} class='w-full h-auto' onclick={() => paused = !paused}>
      <source src={convertFileSrc(content.content_path)} />
      <track kind='captions' />
      Your browser does not support the video tag.
  </Tabs.Content>
</Tabs.Root>
