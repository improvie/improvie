<script lang='ts'>
  import type { RuleType } from '$bindings/rule';
  import { Button } from '$lib/components/ui/button';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input';
  import { defaults, superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import { z } from 'zod';
  import FormError from '../FormError.svelte';

  let { add_rule = $bindable() }: { add_rule: (rule: RuleType) => void } = $props();

  const formSchema = z.object({
    times: z.number().int().nonnegative().default('' as unknown as number),
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

    add_rule({
      type: 'Loop',
      data: {
        times: $formData.times,
        rules: [],
      },
    });
  }
</script>

<form method='POST' use:enhance onsubmit={handleSubmit}>
  <Form.Field {form} name='times'>
    <Form.Control>
      {#snippet children({ props })}
        <div class='grid grid-cols-7 items-center gap-4'>
          <Form.Label class='text-right col-span-2'>Times</Form.Label>
          <Input class='col-span-5' bind:value={$formData.times} type='number' {...props} />
        </div>
      {/snippet}
    </Form.Control>
  </Form.Field>
  <div class='my-4'></div>
  <FormError {form} />
  <div class='my-4'></div>
  <Button type='submit'>Submit</Button>
</form>
