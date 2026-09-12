import { schema } from "../../../../../src/syncular.generated";
import { createTauriSyncClient } from "@syncular/tauri";

export const client = await createTauriSyncClient({ schema });
