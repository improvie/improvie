<script module>
  export { default as PlaylistPlayer } from './Player.svelte';
</script>

<script lang='ts'>
  import type { CurrentRule } from '$lib/stores/track';
  import * as Card from '$lib/components/ui/card/index.js';
  import { current_rule_formats } from '$lib/stores/track';
  import { cn } from '$lib/utils';

  let { open = $bindable() }: { open: boolean } = $props();

  // TODO: youtube music みたいに formats の動画たちをここで見れるようにする?
  // 詳細のエリアの方でもうformatsの見れるようにするからそれをcomponentにして再利用する?
  // 一旦これはあとで考える。
  // eslint-disable-next-line no-unused-vars, unused-imports/no-unused-vars
  const current: CurrentRule = $derived.by(() => {
    if ($current_rule_formats === undefined) {
      return {
        rules: [],
        idx: 0,
      };
    }
    return $current_rule_formats;
  });

</script>

<Card.Root class={cn('container w-1/3 select-none h-[90dvh] transition-all py-6', open || 'hidden')}>
  <p>Todo</p>
</Card.Root>
