import {Button} from '#/components/ui/button';
import {
	Select,
	SelectTrigger,
	SelectValue,
	SelectContent,
	SelectItem,
} from '#/components/ui/select';

const PAGE_SIZES = ['25', '50', '100', '200'];

/** Rows-per-page select + previous/next pager shared by paged tables. */
export function TablePagination({
	total,
	unit,
	pageSize,
	onPageSize,
	pageIndex,
	pageCount,
	onGoto,
}: {
	total: number;
	unit: string;
	pageSize: number;
	onPageSize: (n: number) => void;
	pageIndex: number;
	pageCount: number;
	onGoto: (n: number) => void;
}) {
	return (
		<div className="flex flex-col items-center justify-between gap-4 border-t  pt-3 text-xs text-muted-foreground sm:flex-row">
			<span>
				{total} {total === 1 ? unit : `${unit}s`} total
			</span>

			<div className="flex items-center gap-4">
				<div className="flex items-center gap-2">
					<span className="whitespace-nowrap">Rows per page</span>
					<Select
						value={String(pageSize)}
						onValueChange={val => {
							if (val) {
								onPageSize(Number(val));
								onGoto(0);
							}
						}}>
						<SelectTrigger
							size="sm"
							className="h-8 w-[75px] text-xs font-semibold">
							<SelectValue placeholder="50" />
						</SelectTrigger>
						<SelectContent>
							{PAGE_SIZES.map(s => (
								<SelectItem key={s} value={s}>
									{s}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>

				<span className="font-mono whitespace-nowrap">
					Page {pageIndex + 1} of {pageCount}
				</span>

				<div className="flex gap-2">
					<Button
						variant="outline"
						size="sm"
						onClick={() => onGoto(Math.max(0, pageIndex - 1))}
						disabled={pageIndex === 0}
						className="h-8 px-3 text-xs font-semibold">
						Previous
					</Button>
					<Button
						variant="outline"
						size="sm"
						onClick={() => onGoto(Math.min(pageCount - 1, pageIndex + 1))}
						disabled={pageIndex + 1 >= pageCount}
						className="h-8 px-3 text-xs font-semibold">
						Next
					</Button>
				</div>
			</div>
		</div>
	);
}
