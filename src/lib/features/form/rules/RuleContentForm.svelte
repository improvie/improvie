<script lang='ts'>
  import type { RuleType } from '$lib/types/rules';
  import { Button } from '$lib/components/ui/button';
  import * as Form from '$lib/components/ui/form/index.js';
  import ContentPicker from '$lib/features/combobox/ContentPicker.svelte';
  import { defaults, superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import { z } from 'zod';
  import FormError from '../FormError.svelte';

  let { rules = $bindable() }: { rules: RuleType[] } = $props();

  $inspect(rules);

  const formSchema = z.object({
    content_id: z.string().nonempty(),
  });

  const form = superForm(defaults(zod(formSchema)), {
    SPA: true,
    validators: zod(formSchema),
    resetForm: false,
  });

  const { form: formData, enhance, validateForm } = form;

  async function handleSubmit(event: Event) {
    event.preventDefault();
    const result = await validateForm();
    if (!result.valid) {
      return;
    }

    rules.push({
      type: 'Content',
      data: {
        content_id: $formData.content_id,
      },
    });
  }
</script>

<form method='POST' use:enhance onsubmit={handleSubmit}>
  <Form.Field {form} name='content_id'>
    <Form.Control>
      {#snippet children()}
        <Form.Label class='text-right'>Content</Form.Label>
        <ContentPicker bind:value={$formData.content_id} />
      {/snippet}
    </Form.Control>
  </Form.Field>
  <div class='my-4'></div>
  <FormError {form} />
  <div class='my-4'></div>
  <Button type='submit'>Submit</Button>
</form>
