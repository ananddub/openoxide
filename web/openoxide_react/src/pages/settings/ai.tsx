import {createFileRoute, Navigate} from '@tanstack/react-router';
import { Plus, Sparkles } from "lucide-react";
import { useState } from "react";
import { toast } from "sonner";
import { Button } from "#/components/ui/button";
import { AiProviderDialog } from "#/components/settings/ai/ai-provider-dialog";
import { deleteAiSetting } from "#/components/settings/ai/ai-api";
import { AiSettingsList } from "#/components/settings/ai/ai-settings-list";
import type { AiSetting } from "#/components/settings/ai/ai-types";
import { useAiSettingItems, useConnectMutation } from "#/api/live";

export const Route = createFileRoute("/_app/settings/ai")({
  component: () => (
		<Navigate to="/settings" search={{tab: 'ai'}} replace />
	)
});

export function AiSettingsPage() {
  const itemsQuery = useAiSettingItems();
  const items = (itemsQuery.data ?? []) as AiSetting[];
  const [open, setOpen] = useState(false);
  const [editing, setEditing] = useState<AiSetting | null>(null);

  // Cache invalidation comes from the table-watch stream (ai_settings
  // table → refetch while this page is mounted).
  const deleteMutation = useConnectMutation(
    async (id: number) => {
      await deleteAiSetting(id);
      return id;
    },
    {
      onSuccess: () => {
        toast.success("AI configuration deleted");
      },
      onError: (error: unknown) => {
        toast.error(
          error instanceof Error
            ? error.message
            : "Failed to delete AI configuration",
        );
      },
    },
  );

  const create = () => {
    setEditing(null);
    setOpen(true);
  };

  const edit = (item: AiSetting) => {
    setEditing(item);
    setOpen(true);
  };

  const remove = async (id: number) => {
    if (!confirm("Delete this AI configuration?")) return;
    await deleteMutation.mutateAsync(id);
  };

  return (
    <div className="flex w-full animate-in flex-col gap-6 duration-200 fade-in">
      <div className="flex flex-wrap items-start justify-between gap-4 sm:items-center">
        <div className="flex flex-col gap-0.5">
          <h2 className="text-sm font-medium text-foreground">AI</h2>
          <p className="text-xs text-muted-foreground">
            LLM providers (OpenAI, Anthropic, Gemini, Ollama) for container log
            analysis &amp; template generation.
          </p>
        </div>
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-1.5 rounded-xl bg-muted/40 px-3 py-1.5 text-xs font-medium text-muted-foreground">
            <Sparkles className="size-3.5 text-amber-500" />
            <span>{items.filter((i) => i.is_enabled).length} active</span>
          </div>
          <Button onClick={create} className="gap-1.5 px-4 text-sm">
            <Plus className="size-4" /> Add provider
          </Button>
        </div>
      </div>

      <AiSettingsList
        items={items}
        loading={itemsQuery.isLoading}
        onCreate={create}
        onEdit={edit}
        onDelete={remove}
      />

      {/* Add/Edit Modal */}
      <AiProviderDialog
        editing={editing}
        open={open}
        onOpenChange={setOpen}
        onSaved={async () => {
          /* table-watch invalidates ['ai_settings'] → refetch */
        }}
      />
    </div>
  );
}
