<script lang='ts' module>
  import type { CheckBoxFormSchema } from './CheckBoxFormField.svelte';
  import type { ContentPickFormSchema } from './ContentPickFormField.svelte';
  import type { IntFormSchema, UintFormSchema } from './NumberFormField.svelte';
  import type { RangeFormSchema } from './RangeFormField.svelte';
  import type { StringFormSchema } from './StringFormField.svelte';
  import { defaults, superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';

  import z from 'zod';

  type TypeMap = {
    uint: number;
    int: number;
    string: string;
    checkbox: boolean;
    range: [number, number];
    content_pick: string;
  };

  export type CommonFieldSchema<T extends keyof TypeMap, P = undefined> = {
    type: T;
    label: string;
    props?: P;
  };

  export type AllCommonFieldSchema
    = | UintFormSchema
      | IntFormSchema
      | StringFormSchema
      | CheckBoxFormSchema
      | RangeFormSchema
      | ContentPickFormSchema;

  export type CommonFormSchema = Record<string, AllCommonFieldSchema>;

  export function defineSchema<T extends Record<string, AllCommonFieldSchema>>(schema: T): T {
    return schema;
  }

  export type CommonSchemaToDataType<T extends Record<string, AllCommonFieldSchema>> = {
    [K in keyof T]: TypeMap[T[K]['type']];
  };
  // export type CommonSchemaToDataType<T extends Record<string, CommonFieldSchema<keyof TypeMap>>> = {
  //   [K in keyof T]: T[K]['type'] extends keyof TypeMap ? TypeMap[T[K]['type']] : never;
  // };

  export function createForm(schema: CommonFormSchema) {
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
