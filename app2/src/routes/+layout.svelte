<script lang="ts">
import "../app.css"
import { onMount } from "svelte"
import { Effect, Fiber } from "effect"
import { chainsQuery } from "$lib/queries/chains.svelte"
import Sidebar from "$lib/components/layout/Sidebar/index.svelte"
import AppErrors from "$lib/components/layout/AppErrors/index.svelte"
import { ENV } from "$lib/constants"
import { wallets } from "$lib/stores/wallets.svelte"
import Wallet from "$lib/components/ui/Wallet/index.svelte"
import SettingsModal from "$lib/components/SettingsModal.svelte"
import { uiStore } from "$lib/stores/ui.svelte"

let { children } = $props()

onMount(() => {
  const fiber = Effect.runFork(chainsQuery(ENV()))
  return () => Effect.runPromise(Fiber.interrupt(fiber))
})

$effect(() => {
  console.log(wallets.evmAddress)
  console.log(wallets.cosmosAddress)
  console.log(wallets.aptosAddress)
})
</script>

<div class="grid grid-cols-[auto_1fr] min-h-[100svh] w-screen">
  <aside class="fixed top-0 left-0 bottom-0 w-64 dark:bg-zinc-950 shadow overflow-auto border-r border-zinc-900">
    <Sidebar/>
  </aside>
  
  <!-- Main content area: Has margin to clear fixed sidebar -->
  <main class="col-start-2 ml-64 max-w-[calc(100vw-calc(var(--spacing)*64))]">
    <AppErrors/>
    {@render children()}
  </main>
</div>
<Wallet/>
<SettingsModal 
  isOpen={uiStore.settingsModalOpen} 
  onClose={() => uiStore.closeSettingsModal()}
/>
