<script lang="ts">
	import { prevent_display_sleep, prevent_system_sleep, unblock } from 'tauri-plugin-nosleep-api'

	let response = ''

	function updateResponse(returnValue) {
		response += `[${new Date().toLocaleTimeString()}]` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + '<br>'
	}

	function _blockPreventUserIdleDisplaySleep() {
		prevent_display_sleep().then(() => updateResponse("OK")).catch(updateResponse)
	}

	function _blockPreventUserIdleSystemSleep() {
		prevent_system_sleep().then(() => updateResponse("OK")).catch(updateResponse)
	}

	function _unblock() {
		unblock().then(() => updateResponse("OK")).catch(updateResponse);
	}
</script>

<div>
	<button on:click="{_blockPreventUserIdleDisplaySleep}">Prevent User Idle Display Sleep</button>
	<button on:click="{_blockPreventUserIdleSystemSleep}">Prevent User Idle System Sleep</button>
	<button on:click="{_unblock}">Unblock</button>
	<div>{@html response}</div>
</div>
