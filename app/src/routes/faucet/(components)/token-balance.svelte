<script lang="ts">
import { userBalancesQuery } from "$lib/queries/balance"
import type { Chain, UserAddressCosmos } from "$lib/types.ts"
import Precise from "$lib/components/precise.svelte"

export let chains: Array<Chain>
export let userAddrCosmos: UserAddressCosmos
export let symbol: string

let chain = chains.filter(c => c.chain_id === "union-testnet-8")
$: userBalances = userBalancesQuery({
  userAddr: { cosmos: userAddrCosmos, evm: null, aptos: null },
  chains: chain,
  connected: true
})
$: unionBalances = $userBalances.at(0)?.data ?? []
$: asset = unionBalances.find(balance => balance.symbol.toLowerCase() === symbol.toLowerCase())
</script>

{#if asset}
  <span class=""><Precise chain={chain[0]} {asset} showSymbol displayDecimals={6} /></span>
{/if}
