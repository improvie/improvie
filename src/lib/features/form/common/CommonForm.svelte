<script lang='ts' module>
  import type { SuperForm } from 'sveltekit-superforms';
  import { defaults, superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import z from 'zod';
  import CheckBoxFormField, { CheckBoxFormSchema } from './CheckBoxFormField.svelte';
  import { ContentPickFormSchema } from './ContentPickFormField.svelte';
  import { IntFormSchema, UintFormSchema } from './NumberFormField.svelte';
  import { RangeFormSchema } from './RangeFormField.svelte';
  import { StringFormSchema } from './StringFormField.svelte';

  // eslint-disable-next-line unused-imports/no-unused-vars
  const fieldSchema = z.union([
    UintFormSchema,
    IntFormSchema,
    CheckBoxFormSchema,
    StringFormSchema,
    ContentPickFormSchema,
    RangeFormSchema,
  ]);
  export type FieldSchema = z.infer<typeof fieldSchema>;

  // フォーム全体のスキーマ型
  export type FormSchema = Record<string, FieldSchema>;

  export function createForm<T extends FormSchema>(schema: T) {
    // 各項目の zod 型を自動生成
    const zodShape: Record<string, z.ZodTypeAny> = {};
    for (const [key, def] of Object.entries(schema)) {
      switch (def.type) {
        case 'uint':
          zodShape[key] = z.number().int().nonnegative().min(def.props?.min ?? 0).max(def.props?.max ?? Number.MAX_SAFE_INTEGER).default(def.props?.min ?? 0);
          break;
        case 'int':
          zodShape[key] = z.number().int().min(def.props?.min ?? Number.MIN_SAFE_INTEGER).max(def.props?.max ?? Number.MAX_SAFE_INTEGER).default(def.props?.min ?? 0);
          break;
        case 'checkbox':
          zodShape[key] = z.boolean().default(def.props?.default ?? false);
          break;
        case 'string': {
          let stringZod = z.string();
          if (def.props?.minLength)
            stringZod = stringZod.min(def.props.minLength);
          if (def.props?.maxLength)
            stringZod = stringZod.max(def.props.maxLength);
          zodShape[key] = stringZod;
          break;
        }
        case 'content_pick':
          zodShape[key] = z.string().nonempty().default('');
          break;
        case 'range':
          zodShape[key] = z.tuple([
            z.number().int().nonnegative().min(def.props?.min ?? 0).max(def.props?.max ?? Number.MAX_SAFE_INTEGER),
            z.number().int().nonnegative().min(def.props?.min ?? 0).max(def.props?.max ?? Number.MAX_SAFE_INTEGER),
          ]).default([def.props?.default?.[0] ?? 0, def.props?.default?.[1] ?? 100]);
          break;
        default:
          throw new Error(`Unknown type: ${def}`);
      }
    }
    const formSchema = zod(z.object(zodShape));

    return superForm(defaults(formSchema), {
      SPA: true,
      validators: formSchema,
      resetForm: false,
    });
  }

</script>

<script lang='ts'>
  import { Button } from '$lib/components/ui/button';
  import * as Form from '$lib/components/ui/form/index.js';
  import FormError from '../FormError.svelte';
  import NumberFormField from './NumberFormField.svelte';
  import RangeFormField from './RangeFormField.svelte';
  import StringFormField from './StringFormField.svelte';

  const {
    schema,
    form,
    handle,
  }: {
    schema: FormSchema;
    form: SuperForm<Record<string, any>>;
    handle: (data: any) => void;
    values?: Record<string, any>;
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
