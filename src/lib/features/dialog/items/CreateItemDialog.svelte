<script lang="ts">
  import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { open } from "@tauri-apps/plugin-dialog";
  import { Label } from "$lib/components/ui/label/index.js";
  import { t } from "$lib/translations/translations";
  import { ImportIcon } from "lucide-svelte";

  let title_value: string = $state("");
  let description_value: string | null = $state(null);
  let file_value: string | null = $state(null);

  async function submit() {
    // TODO: call create_content function
  }

  async function select_file() {
    const file = await open({
      title: "Audio or Video",
      multiple: false,
      directory: false,
      filters: [
        {
          name: "Audio or Video",
          extensions: ["mp4", "mp3", "wav"],
        },
      ],
    });
    file_value = file;
    if (file != null && title_value == "") {
      title_value = file.replace(/^.*[\\/]/, "");
    }
  }
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
          spellcheck="false"
          id="title"
          bind:value={title_value}
          placeholder={$t("common.items.import.auto_title")}
          class="col-span-4"
        />
      </div>
      <div class="grid grid-cols-5 items-center gap-4">
        <Label for="description" class="text-right">Description</Label>
        <Input
          spellcheck="false"
          id="description"
          bind:value={description_value}
          placeholder="Option"
          class="col-span-4"
        />
      </div>

      <div class="grid grid-cols-5 items-center gap-4">
        <Label for="file" class="text-right">File</Label>
        <Button
          variant="outline"
          id="file"
          class="col-span-4"
          onclick={() => select_file()}
        >
          {file_value ? file_value : "Select file"}
        </Button>
      </div>
    </div>
    <Dialog.Footer>
      <Button type="submit" onclick={async () => await submit()}>Submit</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
