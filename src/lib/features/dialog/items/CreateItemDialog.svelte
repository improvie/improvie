<script lang="ts">
  import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { t } from "$lib/translations/translations";
  import { ImportIcon } from "lucide-svelte";
</script>

<Dialog.Root>
  <Dialog.Trigger class={buttonVariants({ variant: "outline" })}>
    <ImportIcon class="mr-2 size-4" />Add Item
  </Dialog.Trigger>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Add Item</Dialog.Title>
      <Dialog.Description>Import audio or video</Dialog.Description>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid grid-cols-5 items-center gap-4">
        <Label for="title" class="text-right">Title</Label>
        <Input
          id="title"
          placeholder={$t("common.items.import.auto_title")}
          class="col-span-4"
        />
      </div>
      <div class="grid grid-cols-5 items-center gap-4">
        <Label for="description" class="text-right">Description</Label>
        <Input id="description" class="col-span-4" />
      </div>

      <div class="grid grid-cols-5 items-center gap-4">
        <Label for="item" class="text-right">Import file</Label>
        <Input id="item" class="col-span-4" type="file" />
      </div>
    </div>
    <Dialog.Footer>
      <Button type="submit">Submit</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
