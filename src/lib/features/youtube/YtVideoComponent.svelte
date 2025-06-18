<script lang='ts'>
  import type { VideoDetail } from '$lib/youtube';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import * as RadioGroup from '$lib/components/ui/radio-group/index.js';

  const {
    parent_folder_id,
    detail,
  }: {
    parent_folder_id: string;
    detail: VideoDetail;
  } = $props();

</script>

<Card.Root class='w-full flex flex-row p-4'>
  <div class='flex flex-col w-fit h-30 gap-2'>
    <ImageLoader
      src={detail.thumbnail_url}
      alt={detail.title}
      direction='vertical'
      lazy
    />
    <p class='text-md text-wrap line-clamp-3'>
      {detail.title}
    </p>
  </div>
  <div class='flex flex-col w-full gap-2'>
    <Label for='format'>Select Format</Label>
    <RadioGroup.Root class='flex flex-col gap-2'>
      {#each detail.video_formats as format}
        <div class='flex items-center space-x-2'>
          <RadioGroup.Item value={format.url} id={format.url} />
          <Label for={format.url}>{format.quality_label} - {format.mime_type}</Label>
        </div>
      {/each}
    </RadioGroup.Root>
    <Button
      class='mt-4'
      onclick={() => {
      // Handle import logic here
      }}
    >
      Import Video
    </Button>
  </div>
</Card.Root>
