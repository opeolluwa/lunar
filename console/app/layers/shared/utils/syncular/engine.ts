// // app/syncular/engine.ts

// import type { SyncClientLike } from '@syncular/react'
// import { schema } from './syncular.generated'

// const isTauri = () => {
//   return (
//     typeof window !== 'undefined' &&
//     '__TAURI_INTERNALS__' in window
//   )
// }

// let client: SyncClientLike | null = null

// export async function createEngine(): Promise<SyncClientLike> {
//   if (client) {
//     return client
//   }

//   if (isTauri()) {
//     const { createTauriSyncClient } = await import('@syncular/tauri')

//     client = await createTauriSyncClient({
//       schema,
//     })

//     return client
//   }

//   throw new Error('Browser Syncular engine not implemented')
// }