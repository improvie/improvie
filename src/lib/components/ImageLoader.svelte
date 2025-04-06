<script lang='ts'>
  import { FileXIcon, ImageOffIcon } from '@lucide/svelte';
  import * as Tooltip from './ui/tooltip/index';

  interface Props {
    src: string | undefined;
    alt?: string;
    failed?: boolean;
  }

  let {
    src = $bindable(),
    alt = 'Image not specified',
    failed = $bindable(false),
  }: Props = $props();
</script>

{#if src}
  {#if !failed}
    <img
      src={src}
      alt={alt}
      onerror={() => {
        failed = true;
      }}
      onload={() => {
        failed = false;
      }}
      class='w-full h-auto aspect-video object-cover'
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
      <p>{alt}</p>
    </Tooltip.Content>
  </Tooltip.Root>
{/if}
