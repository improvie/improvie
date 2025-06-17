<script lang='ts'>
  import type { VideoDetail } from '$lib/youtube';

  const {
    parent_folder_id,
    detail,
    onsuccess,
    onerror,
  }: {
    parent_folder_id: string;
    detail: VideoDetail;
    onsuccess: () => void;
    onerror: () => void;
  } = $props();

</script>
