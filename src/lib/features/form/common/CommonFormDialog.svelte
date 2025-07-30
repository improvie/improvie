<script lang='ts' generics='T extends CommonFormSchema'>
  import type { OnChangeFn } from 'vaul-svelte';
  import type { CommonFormProps } from './CommonForm.svelte';
  import type { CommonFormSchema } from './CommonFormSchema.svelte';
  import * as Dialog from '$lib/components/ui/dialog';
  import CommonForm from '$lib/features/form/common/CommonForm.svelte';

  type Props<T extends CommonFormSchema> = CommonFormProps<T> & {
    title: string;
    open?: boolean;
    onOpenChange?: OnChangeFn<boolean>;
  };

  let {
    open = $bindable(false),
    onOpenChange,
    title,
    ...restProps
  }: Props<T> = $props();
</script>

<Dialog.Root bind:open onOpenChange={onOpenChange}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title class='text-center'>{title}</Dialog.Title>
    </Dialog.Header>
    <CommonForm {...restProps} />
  </Dialog.Content>
</Dialog.Root>
