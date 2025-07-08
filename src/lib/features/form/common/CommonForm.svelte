<script lang='ts' module>
  import type { SuperForm } from 'sveltekit-superforms';
  import { defaults, superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import z from 'zod';

  export type SchemaName = 'uint' | 'int' | 'checkbox' | 'string' | 'content_pick';
  const SchemaType: {
    name: SchemaName;
    zod: z.ZodTypeAny;
  }[] = [{
    name: 'uint',
    zod: z.number().int().nonnegative().default('' as unknown as number),
  }, {
    name: 'int',
    zod: z.number().int().default('' as unknown as number),
  }, {
    name: 'checkbox',
    zod: z.boolean().default(false),
  }, {
    name: 'string',
    zod: z.string().nonempty(),
  }, {
    name: 'content_pick',
    zod: z.string().nonempty(),
  }];

  export type FormSchema = {
    [key: string]: {
      type: SchemaName;
      label: string;
    };
  };

  export function createForm(schema: FormSchema): SuperForm<Record<string, any>> {
    const formSchema = z.object(
      Object.fromEntries(
        Object.entries(schema).map(([key, value]) => {
          const type = SchemaType.find(t => t.name === value.type);
          if (!type) {
            throw new Error(`Unknown schema type: ${value.type}`);
          }
          return [key, type.zod];
        }),
      ),
    );

    const form = superForm(defaults(zod(formSchema)), {
      SPA: true,
      validators: zod(formSchema),
      resetForm: false,
    });

    return form;
  }
</script>

<script lang='ts'>
  import { Button } from '$lib/components/ui/button';
  import * as Form from '$lib/components/ui/form/index.js';
  import FormError from '../FormError.svelte';
  import NumberFormField from './NumberFormField.svelte';

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
      {/if}
    </Form.Field>
  {/each}
  <div class='my-4'></div>
  <FormError {form} />
  <div class='my-4'></div>
  <Button type='submit'>Submit</Button>
</form>
