import {createFileRoute} from '@tanstack/react-router';
import {Bell, Send, Mail, MessageSquare} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';

export const Route = createFileRoute('/_app/settings/notifications')({
	component: NotificationsPage,
});

function NotificationsPage() {
	return (
		<div className="p-6 flex flex-col gap-6 max-w-7xl mx-auto w-full animate-in fade-in duration-200">
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-bold text-foreground tracking-tight">Notification Channels</h1>
				<p className="text-xs text-muted-foreground">
					Configure Discord, Slack, Telegram, and Email alerts for deployment failures and server events
				</p>
			</div>

			<div className="grid grid-cols-1 md:grid-cols-3 gap-4">
				<div className="p-5 border rounded-xl bg-card flex flex-col justify-between gap-3">
					<div className="flex items-center gap-3">
						<MessageSquare className="size-5 text-indigo-400" />
						<div>
							<h3 className="text-sm font-bold text-foreground">Discord Webhook</h3>
							<p className="text-[10px] text-muted-foreground">Channel deployment alerts</p>
						</div>
					</div>
					<Button variant="outline" size="sm" className="w-full text-xs h-8">Configure</Button>
				</div>

				<div className="p-5 border rounded-xl bg-card flex flex-col justify-between gap-3">
					<div className="flex items-center gap-3">
						<Send className="size-5 text-sky-400" />
						<div>
							<h3 className="text-sm font-bold text-foreground">Telegram Bot</h3>
							<p className="text-[10px] text-muted-foreground">Instant chat notifications</p>
						</div>
					</div>
					<Button variant="outline" size="sm" className="w-full text-xs h-8">Configure</Button>
				</div>

				<div className="p-5 border rounded-xl bg-card flex flex-col justify-between gap-3">
					<div className="flex items-center gap-3">
						<Mail className="size-5 text-rose-400" />
						<div>
							<h3 className="text-sm font-bold text-foreground">SMTP Email</h3>
							<p className="text-[10px] text-muted-foreground">Critical email alerts</p>
						</div>
					</div>
					<Button variant="outline" size="sm" className="w-full text-xs h-8">Configure</Button>
				</div>
			</div>
		</div>
	);
}
