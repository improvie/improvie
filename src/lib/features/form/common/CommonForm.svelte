<script lang='ts' generics='T extends CommonFormSchema'>
  import type { SuperForm } from 'sveltekit-superforms';
  import type { CommonFormSchema, CommonSchemaToDataType } from './CommonFormSchema.svelte';
  import { Button } from '$lib/components/ui/button';
  import * as Form from '$lib/components/ui/form/index.js';
  import FormError from '../FormError.svelte';
  import CheckBoxFormField from './CheckBoxFormField.svelte';
  import ContentPickFormField from './ContentPickFormField.svelte';
  import FolderPickFormField from './FolderPickFormField.svelte';
  import NumberFormField from './NumberFormField.svelte';
  import RangeFormField from './RangeFormField.svelte';
  import StringFormField from './StringFormField.svelte';

  export type CommonFormProps<T extends CommonFormSchema> = {
    schema: T;
    form: SuperForm<CommonSchemaToDataType<T>>;
    handle: (data: CommonSchemaToDataType<T>) => void;
    handleChange?: (data: CommonSchemaToDataType<T>) => CommonSchemaToDataType<T> | void;
  };

  const {
    schema = $bindable(),
    form,
    handle,
    handleChange,
  }: CommonFormProps<T> = $props();

  const { form: formData, enhance, validateForm } = form;

  async function handleSubmit(event: Event) {
    event.preventDefault();
    const result = await validateForm();
    if (!result.valid) {
      return;
    }

    handle(result.data);
  }

  $effect(() => {
    if (formData) {
      const result = handleChange?.($formData);
      if (result) {
        $formData = result;
      }
    }
  });
</script>

<form method='POST' use:enhance onsubmit={handleSubmit}>
  {#each Object.entries(schema) as [key, value]}
    <Form.Field {form} name={key as never} class='my-3'>
      {#if value.type === 'uint' || value.type === 'int'}
        <NumberFormField
          bind:value={$formData[key] as never}
          label={value.label}
          props={value.props}
        />
      {:else if value.type === 'checkbox'}
        <CheckBoxFormField
          bind:value={$formData[key] as never}
          label={value.label}
          props={value.props}
        />
      {:else if value.type === 'string'}
        <StringFormField
          bind:value={$formData[key] as never}
          label={value.label}
        />
      {:else if value.type === 'content_pick'}
        <ContentPickFormField
          bind:value={$formData[key] as never}
          label={value.label}
        />
      {:else if value.type === 'folder_pick'}
        <FolderPickFormField
          bind:value={$formData[key] as never}
          label={value.label}
        />
      {:else if value.type === 'range'}
        <RangeFormField
          bind:value={$formData[key] as never}
          label={value.label}
          bind:props={value.props}
        />
      {/if}
    </Form.Field>
  {/each}
  <div class='my-4 flex'>
    <FormError {form} />
    <Button type='submit' class='ml-auto'>Submit</Button>
  </div>
</form>
