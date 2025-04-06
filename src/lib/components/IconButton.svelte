<script lang='ts'>
  import type { WithElementRef } from 'bits-ui';
  import type { Snippet } from 'svelte';
  import type { HTMLAnchorAttributes, HTMLButtonAttributes } from 'svelte/elements';
  import type { ButtonVariant } from './ui/button';
  import { Button } from './ui/button';
  import * as Tooltip from './ui/tooltip/index';

  type Props = WithElementRef<HTMLButtonAttributes> &
    WithElementRef<HTMLAnchorAttributes> & {
      variant?: ButtonVariant;
      content: Snippet;
      children: Snippet;
      delayDuration?: number;
    };

  const {
    class: className,
    variant = 'outline',
    type = 'button',
    content,
    children,
    delayDuration = 500,
    ...restProps
  }: Props = $props();

</script>

<Tooltip.Root delayDuration={delayDuration} disableHoverableContent disableCloseOnTriggerClick>
  <Tooltip.Trigger>
    <Button
      type={type}
      variant={variant}
      size='icon'
      class={className}
      {...restProps}
    >
      {@render children()}
    </Button>
  </Tooltip.Trigger>
  <Tooltip.Content>
    {@render content()}
  </Tooltip.Content>
</Tooltip.Root>
