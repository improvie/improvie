<script lang='ts'>
  import type { VideoDetail } from '$lib/youtube';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';

  const {
    parent_folder_id,
    detail,
  }: {
    parent_folder_id: string;
    detail: VideoDetail;
  } = $props();

</script>

<Card.Root class='w-full max-w-sm'>
  <div class='flex items-center h-20 justify-between p-4'>
    <ImageLoader
      src={detail.thumbnail_url}
      alt={detail.title}
      direction='vertical'
      lazy
    />
    <p class='text-xl'>{detail.title}</p>
  </div>
  <Card.Footer class='flex-col gap-2'>
    <Button type='submit' class='w-full'>Download</Button>
  </Card.Footer>
</Card.Root>
