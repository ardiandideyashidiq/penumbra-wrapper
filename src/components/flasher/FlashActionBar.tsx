interface FlashActionBarProps {
  selectedCount: number;
  selectedWithImagesCount: number;
  missingImagesCount: number;
  canFlash: boolean;
  isFlashing: boolean;
  currentFlashIndex: number;
  totalFlashCount: number;
  isConnected: boolean;
  daPath: string | null;
  onFlash: () => void;
}

export function FlashActionBar({
  selectedCount,
  selectedWithImagesCount,
  missingImagesCount,
  canFlash,
  isFlashing,
  currentFlashIndex,
  totalFlashCount,
  isConnected,
  daPath,
  onFlash,
}: FlashActionBarProps) {
  const isDisabled = !canFlash || isFlashing;

  const getTitle = (): string => {
    if (isFlashing) return 'Flashing in progress...';
    if (!daPath) return 'No DA file selected';
    if (!isConnected) return 'Connect a device first';
    if (selectedCount === 0) return 'Select partitions to flash';
    if (missingImagesCount > 0) return `${missingImagesCount} partition(s) missing image files`;
    return 'Flash selected partitions';
  };
  
  return (
    <div className="flex flex-col gap-3 rounded-lg border border-[var(--border)] bg-[var(--surface-alt)] p-4 sm:flex-row sm:items-center sm:justify-between">
      <div className="min-w-0 text-sm text-[var(--text-muted)]">
        {isFlashing ? (
          <span className="block truncate font-semibold text-[var(--accent)]">
            Flashing partition {currentFlashIndex} of {totalFlashCount}...
          </span>
        ) : (
          <>
            <span className="font-semibold text-[var(--text)]">{selectedCount}</span> partitions selected
            {selectedCount > 0 && (
              <span className="ml-2">({selectedWithImagesCount} with images)</span>
            )}
          </>
        )}
      </div>
      <button
        onClick={onFlash}
        disabled={isDisabled}
        title={getTitle()}
        className="w-full rounded-lg bg-[var(--accent)] px-6 py-2 font-medium text-[var(--accent-foreground)] transition-colors hover:bg-[var(--accent-hover)] disabled:cursor-not-allowed disabled:opacity-50 sm:w-auto"
      >
        {isFlashing ? 'Flashing...' : 'Flash Selected'}
      </button>
    </div>
  );
}
