<script lang='ts'>
  import type { VideoDetail } from '$lib/youtube';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import * as RadioGroup from '$lib/components/ui/radio-group/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

  const {
    parent_folder_id,
    detail,
  }: {
    parent_folder_id: string;
    detail: VideoDetail;
  } = $props();

  let downloadUrl = $state<string>(detail.video_formats[0].url);

</script>

<Card.Root class='sm:max-w-md flex flex-row p-4 h-52'>
  <div class='flex flex-col gap-2'>
    <ImageLoader
      src={detail.thumbnail_url}
      alt={detail.title}
      direction='vertical'
      class='h-32'
      lazy
    />
    <p class='text-sm text-wrap line-clamp-2'>
      {detail.title}
    </p>
  </div>
  <div class='flex flex-col justify-between'>
    <ScrollArea class='h-32'>
      <RadioGroup.Root bind:value={downloadUrl} class='flex flex-col gap-2'>
        {#each detail.video_formats as format}
          <div class='flex items-center space-x-2'>
            <RadioGroup.Item value={format.url} id={format.url} />
            <Label for={format.url}>{format.quality_label}</Label>
          </div>
        {/each}
      </RadioGroup.Root>
    </ScrollArea>
    <Button
      onclick={() => {

      }}
    >
      Import Video
    </Button>
  </div>
</Card.Root>
