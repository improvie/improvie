<script lang='ts'>
  // ref: https://animation-svelte.vercel.app/magic/circular-progress-bar
  import { cn } from '$lib/utils';

  const {
    max = 100,
    value = $bindable(0),
    min = 0,
    gaugePrimaryClass,
    gaugeSecondaryClass,
    class: className,
  }: {
    max?: number;
    value?: number;
    min?: number;
    gaugePrimaryClass?: string;
    gaugeSecondaryClass?: string;
    class?: string;
  } = $props();

  const circumference = 2 * Math.PI * 45;
  const percentPx = circumference / 100;
  const currentPercent = $derived(((value - min) / (max - min)) * 100);
</script>

<div
  class={cn('relative size-40 text-2xl font-semibold', className)}
  style:--circle-size='100px'
  style:--circumference={circumference}
  style:--percent-to-px='{percentPx}px'
  style:--gap-percent='5'
  style:--offset-factor='0'
  style:--transition-length='1s'
  style:--transition-step='200ms'
  style:--delay='0s'
  style:--percent-to-deg='3.6deg'
  style='transform: translateZ(0);'
>
  <svg fill='none' class='size-full' stroke-width='2' viewBox='0 0 100 100'>
    {#if currentPercent <= 90 && currentPercent >= 0}
      <circle
        cx='50'
        cy='50'
        r='45'
        stroke-width='10'
        stroke-dashoffset='0'
        stroke-linecap='round'
        stroke-linejoin='round'
        class={cn('opacity-100', gaugeSecondaryClass)}
        style='
          stroke-dasharray: calc(var(--stroke-percent) * var(--percent-to-px)) var(--circumference);
          transform: rotate(calc(1turn - 90deg - (var(--gap-percent) * var(--percent-to-deg) * var(--offset-factor-secondary)))) scaleY(-1);
          transition: all var(--transition-length) ease var(--delay);
          transform-origin:calc(var(--circle-size) / 2) calc(var(--circle-size) / 2);
        '
        style:--stroke-percent={90 - currentPercent}
        style:--offset-factor-secondary='calc(1 - var(--offset-factor))'
      />
    {/if}
    <circle
      cx='50'
      cy='50'
      r='45'
      stroke-width='10'
      stroke-dashoffset='0'
      stroke-linecap='round'
      stroke-linejoin='round'
      class={cn('opacity-100', gaugePrimaryClass)}
      style='
        stroke-dasharray:calc(var(--stroke-percent) * var(--percent-to-px)) var(--circumference);
        transition:var(--transition-length) ease var(--delay),stroke var(--transition-length) ease var(--delay);
        transition-property: stroke-dasharray,transform;
        transform:rotate(calc(-90deg + var(--gap-percent) * var(--offset-factor) * var(--percent-to-deg)));
        transform-origin:calc(var(--circle-size) / 2) calc(var(--circle-size) / 2);
      '
      style:--stroke-percent={currentPercent}
    />
  </svg>
  <span
    data-current-value={currentPercent}
    class='duration-[var(--transition-length)] delay-[var(--delay)] absolute inset-0 m-auto size-fit ease-linear animate-in fade-in'
  >
    {currentPercent.toFixed(0)}
  </span>
</div>
