import { getToastStore, type ToastSettings } from '@skeletonlabs/skeleton';

const toastStore = getToastStore();

const offline: ToastSettings = {
	message: 'You are currently offline.',
	background: 'variant-filled-error',
	autohide: false
};

const online: ToastSettings = {
	message: 'You are back online.',
	background: 'variant-filled-success',
	autohide: true,
	timeout: 3000
};

/**
 * Make a toast for if the PWA is ever brought offline for whatever reason.
 */
export default function offlineToast() {
	window.addEventListener('offline', () => {
		toastStore.clear();
		toastStore.trigger(offline);
	});

	window.addEventListener('online', () => {
		toastStore.clear();
		setTimeout(() => {
			toastStore.trigger(online);
		}, 300);
	});
}
