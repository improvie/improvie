<script lang='ts'>
  import type { PickItem } from '$lib/types/item';
  import type { RuleType } from '$lib/types/rules';
  import { Button } from '$lib/components/ui/button';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input';
  import ContentPicker from '$lib/features/combobox/ContentPicker.svelte';
  import { defaults, superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import { z } from 'zod';
  import FormError from '../FormError.svelte';

  let { add_rule = $bindable() }: { add_rule: (rule: RuleType) => void } = $props();

  const formSchema = z.object({
    content_id: z.string().nonempty(),
    range_start: z.number().int().nonnegative().default('' as unknown as number),
    range_end: z.number().int().nonnegative().default('' as unknown as number),
  });

  const form = superForm(defaults(zod(formSchema)), {
    SPA: true,
    validators: zod(formSchema),
    resetForm: false,
  });

  const { form: formData, enhance, validateForm } = form;

  let open = $state(false);
  let pick_content = $state<PickItem | undefined>();

  $effect(() => {
    if (pick_content) {
      $formData.content_id = pick_content.id;
    }
  });

  async function handleSubmit(event: Event) {
    event.preventDefault();
    const result = await validateForm();
    if (!result.valid) {
      return;
    }

    add_rule({
      type: 'Range',
      data: {
        content_id: $formData.content_id,
        range_start: $formData.range_start,
        range_end: $formData.range_end,
      },
    });
  }
</script>

<ContentPicker bind:content={pick_content} bind:open />

<form method='POST' use:enhance onsubmit={handleSubmit}>
  <Form.Field {form} name='content_id'>
    <Form.Control>
      {#snippet children({ props })}
        <div class='grid grid-cols-5 items-center gap-4'>
          <Form.Label class='text-right col-span-2'>Content</Form.Label>
          <Button
            class='col-span-3'
            variant='outline'
            {...props}
            onclick={() => open = true}
          >{pick_content?.hierarchy_name || 'Select content'}
          </Button>
        </div>
      {/snippet}
    </Form.Control>
  </Form.Field>
  <Form.Field {form} name='range_start'>
    <Form.Control>
      {#snippet children({ props })}
        <div class='grid grid-cols-5 items-center gap-4'>
          <Form.Label class='text-right col-span-2'>Range Start</Form.Label>
          <Input class='col-span-3' bind:value={$formData.range_start} type='number' {...props} />
        </div>
      {/snippet}
    </Form.Control>
  </Form.Field>
  <Form.Field {form} name='range_end'>
    <Form.Control>
      {#snippet children({ props })}
        <div class='grid grid-cols-5 items-center gap-4'>
          <Form.Label class='text-right col-span-2'>Range End</Form.Label>
          <Input class='col-span-3' bind:value={$formData.range_end} type='number' {...props} />
        </div>
      {/snippet}
    </Form.Control>
  </Form.Field>
  <div class='my-4'></div>
  <FormError {form} />
  <div class='my-4'></div>
  <Button type='submit'>Submit</Button>
</form>
