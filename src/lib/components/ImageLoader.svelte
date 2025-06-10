<script lang='ts'>
  import { FileXIcon, ImageOffIcon } from '@lucide/svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import * as Tooltip from './ui/tooltip/index';

  interface Props {
    src: string | undefined | null;
    alt?: string;
    failed?: boolean;
    local?: boolean;
  }

  let {
    src = $bindable(),
    alt = $bindable(),
    failed = $bindable(false),
    local = false,
  }: Props = $props();

  const imageSrc: string | undefined = $derived.by(() => {
    if (src === undefined || src === null) {
      return undefined;
    }
    if (local) {
      return convertFileSrc(src);
    }
    return src;
  });

</script>

{#if imageSrc}
  {#if !failed}
    <img
      src={imageSrc}
      alt={alt}
      onerror={() => {
        failed = true;
      }}
      onload={() => {
        failed = false;
      }}
      class='h-full w-auto aspect-video object-contain'
    />
  {:else}
    <Tooltip.Root delayDuration={500} disableHoverableContent disableCloseOnTriggerClick>
      <Tooltip.Trigger class='w-full h-fit flex justify-center'>
        <FileXIcon class='w-9/16 h-auto aspect-square' />
      </Tooltip.Trigger>
      <Tooltip.Content>
        <p>Image load failed.</p>
      </Tooltip.Content>
    </Tooltip.Root>
  {/if}
{:else}
  <Tooltip.Root delayDuration={500} disableHoverableContent disableCloseOnTriggerClick>
    <Tooltip.Trigger class='w-full h-fit flex justify-center'>
      <ImageOffIcon class='w-9/16 h-auto aspect-square' />
    </Tooltip.Trigger>
    <Tooltip.Content>
      <p>Image not specified.</p>
    </Tooltip.Content>
  </Tooltip.Root>
{/if}
