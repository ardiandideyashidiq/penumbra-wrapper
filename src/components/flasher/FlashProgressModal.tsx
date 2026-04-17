import { useEffect, useState } from 'react';
import { Zap, Loader2, Minus } from 'lucide-react';
import { Progress } from '../ui/progress';

export interface FlashProgressModalProps {
  isOpen: boolean;
  onClose: () => void;
  onMinimize?: () => void;
  currentPartition: string;
  currentIndex: number;
  totalCount: number;
}

export function FlashProgressModal({
  isOpen,
  onClose,
  onMinimize,
  currentPartition,
  currentIndex,
  totalCount,
}: FlashProgressModalProps) {
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    if (isOpen) {
      const timer = setTimeout(() => setIsVisible(true), 0);
      return () => clearTimeout(timer);
    }

    const timer = setTimeout(() => setIsVisible(false), 300);
    return () => clearTimeout(timer);
  }, [isOpen]);

  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, onClose]);

  if (!isVisible && !isOpen) return null;

  const percentage = currentIndex >= totalCount ? 100 : totalCount > 0 ? (currentIndex / totalCount) * 99 : 0;

  return (
    <>
      {/* Backdrop */}
      <div
        className={`fixed inset-0 bg-black/60 z-[100] transition-opacity duration-300 ${
          isOpen ? 'opacity-100' : 'opacity-0'
        }`}
        onClick={onClose}
      />

      {/* Modal */}
      <div
        className={`fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 z-[101] w-full max-w-lg mx-4 transition-all duration-300 ${
          isOpen ? 'opacity-100 scale-100' : 'opacity-0 scale-95'
        }`}
      >
        <div className="bg-[var(--surface)] rounded-lg border-2 border-[var(--accent)] shadow-2xl overflow-hidden">
          {/* Header */}
          <div className="flex items-start gap-3 p-5 border-b border-[var(--border)]">
            <Zap className="w-6 h-6 text-[var(--accent)] flex-shrink-0 mt-0.5" />
            <h2 className="text-xl font-semibold text-[var(--text)] flex-1">Flashing Partitions</h2>
            <div className="flex items-center gap-1">
              {onMinimize && (
                <button
                  onClick={onMinimize}
                  className="p-1 hover:bg-[var(--surface-alt)] rounded transition-colors"
                  title="Minimize to sidebar"
                >
                  <Minus className="w-5 h-5 text-[var(--text-muted)]" />
                </button>
              )}
            </div>
          </div>

          {/* Content */}
          <div className="p-5 text-[var(--text)] space-y-4">
            <p className="text-sm text-[var(--text-muted)]">
              Flashing partition {currentIndex} of {totalCount}...
            </p>

            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <span className="text-[var(--text-muted)]">Current:</span>
                <span className="font-mono text-sm text-[var(--accent)]">
                  {currentPartition}
                </span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-[var(--text-muted)]">Progress:</span>
                <span className="font-mono text-sm">
                  {currentIndex} / {totalCount}
                </span>
              </div>
            </div>

            {/* Progress Bar */}
            <div className="space-y-3 p-4 bg-[var(--surface-alt)] rounded-lg border border-[var(--border)]">
              <div className="flex items-center gap-2 text-sm font-medium text-[var(--accent)]">
                <Loader2 className="w-4 h-4 animate-spin" />
                <span>Flashing...</span>
              </div>

              <Progress value={percentage} className="animate-pulse" />

              <div className="flex items-center justify-between text-xs text-[var(--text-subtle)]">
                <span>{currentPartition}</span>
                <span>{percentage.toFixed(0)}%</span>
              </div>
            </div>
          </div>

          {/* Footer */}
          <div className="flex items-center justify-between gap-3 p-5 border-t border-[var(--border)] bg-[var(--surface)]">
            <button
              onClick={onClose}
              className="px-5 py-2 bg-[var(--danger)] hover:bg-[var(--danger-hover)] text-white rounded transition-colors"
            >
              Cancel
            </button>
            {onMinimize && (
              <button
                onClick={onMinimize}
                className="px-5 py-2 bg-[var(--primary)] hover:bg-[var(--primary-hover)] text-[var(--primary-foreground)] rounded transition-colors"
              >
                Minimize
              </button>
            )}
          </div>
        </div>
      </div>
    </>
  );
}