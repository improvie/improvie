<script lang='ts'>
  import { cn } from '$lib/utils';
  import { FileXIcon, ImageOffIcon, LoaderIcon } from '@lucide/svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { tv } from 'tailwind-variants';
  import * as Tooltip from './ui/tooltip/index';

  interface Props {
    src: string | undefined | null;
    alt?: string;
    direction?: 'horizontal' | 'vertical';
    loading?: boolean;
    failed?: boolean;
    local?: boolean;
    class?: string;
  }

  const variants = tv({
    variants: {
      direction: {
        horizontal: 'w-full',
        vertical: 'h-full',
      },
      target: {
        img: 'aspect-video object-cover',
        trigger: 'flex justify-center',
      },
    },
  });

  const iconVariants = tv({
    base: 'aspect-square',
    variants: {
      direction: {
        horizontal: 'w-9/16 h-auto',
        vertical: 'h-9/16 w-auto',
      },
    },
  });

  let {
    src,
    alt,
    direction = 'horizontal',
    failed = $bindable(false),
    loading = false,
    local = false,
    class: className,
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
      class={cn(variants({ direction, target: 'img' }), className)}
    />
  {:else}
    <Tooltip.Root delayDuration={500} disableHoverableContent disableCloseOnTriggerClick>
      <Tooltip.Trigger class={variants({ direction, target: 'trigger' })}>
        <FileXIcon class={iconVariants({ direction })} />
      </Tooltip.Trigger>
      <Tooltip.Content>
        <p>Image load failed.</p>
      </Tooltip.Content>
    </Tooltip.Root>
  {/if}
{:else}
  {#if loading}
    <Tooltip.Root delayDuration={500} disableHoverableContent disableCloseOnTriggerClick>
      <Tooltip.Trigger class={variants({ direction, target: 'trigger' })}>
        <LoaderIcon class={iconVariants({ direction })} />
      </Tooltip.Trigger>
      <Tooltip.Content>
        <p>Image is loading...</p>
      </Tooltip.Content>
    </Tooltip.Root>
  {:else}
    <Tooltip.Root delayDuration={500} disableHoverableContent disableCloseOnTriggerClick>
      <Tooltip.Trigger class={variants({ direction, target: 'trigger' })}>
        <ImageOffIcon class={iconVariants({ direction })} />
      </Tooltip.Trigger>
      <Tooltip.Content>
        <p>Image not specified.</p>
      </Tooltip.Content>
    </Tooltip.Root>
  {/if}
{/if}
