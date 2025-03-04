<!-- ChainSelector.svelte -->
<script lang="ts">
import type { RawIntentsStore } from "$lib/components/TransferCube/transfer/raw-intents.ts"
import type { CubeFaces } from "$lib/components/TransferCube/components/Cube/types.ts"
import { TRANSFER_DEBUG } from "$lib/components/TransferCube/transfer/config.ts"
import type { Chain } from "$lib/types.ts"
import { page } from "$app/stores"
import ChainDetails from "$lib/chain-details.svelte"

interface Props {
  rawIntents: RawIntentsStore
  chains: Array<Chain>
  rotateTo: (face: CubeFaces) => void
  selected: "source" | "destination"
}

export let rawIntents: Props["rawIntents"]
export let chains: Props["chains"]
export let rotateTo: Props["rotateTo"]
export let selected: Props["selected"]

const enabledChains = chains.filter(chain => chain.features[0].transfer_submission)
let expandedChainId: string | null = null

function setChain(selected: "source" | "destination", chainId: string) {
  rawIntents.updateField(selected, chainId)
  rotateTo("intentFace")
}

function toggleExpand(chainId: string) {
  expandedChainId = expandedChainId === chainId ? null : chainId
}
</script>

<div class="flex flex-col h-full w-full">
  <!-- Title Bar -->
  <div class="text-primary p-2 flex items-center justify-between border-b-2">
    <span class="font-bold uppercase">{selected} chain</span>
    <button
            class="border-2 h-6 w-6 flex items-center justify-center"
            on:click={() => rotateTo("intentFace")}
    >✕
    </button>
  </div>

  <!-- Chain List -->
  <div class="flex flex-col h-full overflow-y-scroll">
      {#each enabledChains as chain}
        <div>
          <button
                  class="px-2 py-1 w-full hover:bg-neutral-400 dark:hover:bg-neutral-800 text-md flex justify-start items-center"
                  on:click={() => setChain(selected, chain.chain_id)}
          >
            <div class="flex items-center gap-2 font-supermolot font-semibold">
              <ChainDetails {chains} chainId={chain.chain_id}/>
            </div>
          </button>

          <!-- Expanded Info Panel -->
          {#if expandedChainId === chain.chain_id}
            <div class="">
              <div class="grid grid-cols-2 gap-2 text-sm">
                <div class="border-2 border-black p-2">
                  <h4 class="font-bold mb-1">Network Info</h4>
                  <p>Chain ID: {chain.chain_id}</p>
                  <p>Type: {chain.rpc_type}</p>
                  <p>Prefix: {chain.addr_prefix}</p>
                </div>
                {#if !TRANSFER_DEBUG}
                  <div class="border-2 border-black p-2">
                    <h4 class="font-bold mb-1">Status</h4>
                    <p>Enabled: {chain.enabled ? '✓' : '✗'}</p>
                    <p>Staging: {chain.enabled_staging ? '✓' : '✗'}</p>
                  </div>
                {/if}
                {#if chain.explorers?.length}
                  <div class="col-span-2 border-2 border-black p-2">
                    <h4 class="font-bold mb-1">Explorers</h4>
                    {#each chain.explorers as explorer}
                      <a href={explorer.tx_url} class="text-xs truncate">
                        {explorer.tx_url.split('/')[2]}
                      </a>
                    {/each}
                  </div>
                {/if}
              </div>
            </div>
          {/if}
        </div>
      {/each}
  </div>
</div>
