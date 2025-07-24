<script lang='ts'>
  import { action_get_recent_contents } from '$lib/action/recents';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import ScrollArea from '$lib/components/ui/scroll-area/scroll-area.svelte';
  import { contents } from '$lib/stores/items/content';

  let recentContents: string[] = $state([]);

  async function init() {
    recentContents = await action_get_recent_contents(30);
  }

  $effect(() => {
    init();
  });

  function onclick(contentId: string) {
  }

</script>

<main class='flex flex-col w-full h-dvh items-center gap-4'>
  <div class='flex flex-col container'>
    <div class='flex justify-between'>
      <h2 class='text-3xl m-2 font-bold'>
        最近みたもの
      </h2>
      <div>
      </div>
    </div>
    <ScrollArea class='whitespace-nowrap rounded-md border container' orientation='horizontal'>
      <div class='h-74 flex flex-col flex-wrap gap-6 p-4'>
        {#each recentContents as contentId}
          {@const content = contents.get(contentId)}
          {#if content !== undefined}
            <div
              role={content.title}
              class='h-18 w-90 gap-1 flex flex-row'
              onclick={() => onclick(contentId)}
            >
              <ImageLoader local direction='vertical' class='rounded-sm border' src={content.thumbnail_path} />
              <p class='text-wrap line-clamp-3'>{content.title}</p>
            </div>
          {/if}
        {/each}
      </div>
    </ScrollArea>
  </div>
</main>
