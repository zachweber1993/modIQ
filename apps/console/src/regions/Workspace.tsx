import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { Button } from "@/components/ui/button";
import { submitAssessment } from "@/engine";

/**
 * Workspace — Intake (Phase 2, Sprint 21). Reviewing arrives once a
 * submission completes, but this phase renders only a bare
 * success/failure acknowledgment, never Overview or Finding content —
 * that presentation is Phase 3's Navigation Realization and Workspace
 * Realization work, not this phase's.
 *
 * `status` is not a fourth workspace state alongside Intake/Assessing/
 * Reviewing. It is transient UI state *within* Intake: the user has
 * not left Intake by choosing a folder, only by receiving a result.
 * No Assessing-state view exists here — the pending condition carries
 * no phase information, no percentage, no Finding content, only "a
 * request is in flight."
 */
type IntakeStatus = "idle" | "submitting" | "completed" | "failed";

export function Workspace() {
  const [status, setStatus] = useState<IntakeStatus>("idle");
  const [selectedPath, setSelectedPath] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const chooseFolder = async () => {
    const path = await open({ directory: true, multiple: false });
    // Cancelled: no state change, no error — fully reversible, no
    // ceremony (ASSESSMENT_INTAKE_AND_UPLOAD.md §6).
    if (!path || Array.isArray(path)) return;

    setSelectedPath(path);
    setError(null);
    setStatus("submitting");

    try {
      await submitAssessment(path);
      setStatus("completed");
    } catch (err) {
      setError(String(err));
      setStatus("failed");
    }
  };

  if (status === "completed") {
    return (
      <div className="flex h-full items-center justify-center text-foreground">
        <p>Assessment complete.</p>
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col items-center justify-center gap-3 text-center">
      {selectedPath && (
        <p className="text-sm text-muted-foreground">Included: {selectedPath}</p>
      )}
      {status === "failed" && (
        <p className="text-sm text-destructive">
          Something went wrong: {error}
        </p>
      )}
      <Button onClick={chooseFolder} disabled={status === "submitting"}>
        {status === "submitting" ? "Submitting…" : "Choose a mod folder"}
      </Button>
    </div>
  );
}
