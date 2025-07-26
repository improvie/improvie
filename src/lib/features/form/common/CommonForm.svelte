<script lang='ts'>
  import type { SuperForm } from 'sveltekit-superforms';
  import type { CommonFormSchema } from './CommonFormSchema.svelte';
  import { Button } from '$lib/components/ui/button';
  import * as Form from '$lib/components/ui/form/index.js';
  import FormError from '../FormError.svelte';
  import CheckBoxFormField from './CheckBoxFormField.svelte';
  import ContentPickFormField from './ContentPickFormField.svelte';
  import NumberFormField from './NumberFormField.svelte';
  import RangeFormField from './RangeFormField.svelte';
  import StringFormField from './StringFormField.svelte';

  const {
    schema,
    form,
    handle,
  }: {
    schema: CommonFormSchema;
    form: SuperForm<Record<string, any>>;
    handle: (data: any) => void;
  } = $props();

  const { form: formData, enhance, validateForm } = form;

  async function handleSubmit(event: Event) {
    event.preventDefault();
    const result = await validateForm();
    if (!result.valid) {
      return;
    }

    handle(result.data);
  }
</script>

<form method='POST' use:enhance onsubmit={handleSubmit}>
  {#each Object.entries(schema) as [key, value]}
    <Form.Field {form} name={key}>
      {#if value.type === 'uint' || value.type === 'int'}
        <NumberFormField
          bind:value={$formData[key]}
          label={value.label}
        />
      {:else if value.type === 'checkbox'}
        <CheckBoxFormField
          bind:value={$formData[key]}
          label={value.label}
        />
      {:else if value.type === 'string'}
        <StringFormField
          bind:value={$formData[key]}
          label={value.label}
        />
      {:else if value.type === 'content_pick'}
        <ContentPickFormField
          bind:value={$formData[key]}
          label={value.label}
        />
      {:else if value.type === 'range'}
        <RangeFormField
          bind:value={$formData[key]}
          label={value.label}
          props={value.props}
        />
      {/if}
    </Form.Field>
  {/each}
  <div class='my-4'></div>
  <FormError {form} />
  <div class='my-4'></div>
  <Button type='submit'>Submit</Button>
</form>
