<script lang='ts'>
  import {
    Button,
  } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { defaults, superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import { z } from 'zod';
  import FormError from '../form/FormError.svelte';

  let {
    data = $bindable(),
  }: {
    data: {
      now_name: string;
      update_fn: (name: string) => void;
    } | undefined;
  } = $props();

  let open = $derived(data !== undefined);

  function onOpenChange(open: boolean) {
    if (!open) {
      data = undefined;
    }
  }

  const formSchema = z.object({
    name: z.string().nonempty(),
  });

  const form = superForm(defaults(zod(formSchema)), {
    SPA: true,
    validators: zod(formSchema),
    resetForm: false,
  });

  const { form: formData, enhance, validateForm } = form;

  $effect(() => {
    if (data !== undefined) {
      $formData.name = data.now_name;
    }
  });

  $effect(() => {
    if (!open) {
      form.reset();
    }
  });

  async function handleSubmit(event: Event) {
    event.preventDefault();
    const result = await validateForm();

    if (!result.valid) {
      return;
    }

    data?.update_fn($formData.name);

    setTimeout(() => {
      open = false;
    }, 10);
  }

</script>

<Dialog.Root bind:open onOpenChange={onOpenChange}>
  <Dialog.Content class='sm:max-w-[425px]'>
    <form method='POST' use:enhance onsubmit={handleSubmit}>
      <Dialog.Header>
        <Dialog.Title>Rename</Dialog.Title>
      </Dialog.Header>
      <div class='grid gap-4 py-4'>
        <Form.Field {form} name='name'>
          <Form.Control>
            {#snippet children({ props })}
              <div class='grid grid-cols-5 items-center gap-4'>
                <Form.Label class='text-right'>Name</Form.Label>
                <Input
                  class='col-span-4'
                  {...props}
                  bind:value={$formData.name}
                  spellcheck='false'
                />
              </div>
            {/snippet}
          </Form.Control>
        </Form.Field>
      </div>
      <Dialog.Footer>
        <FormError {form} />
        <Button type='submit'>Save changes</Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
