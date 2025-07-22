<script lang="ts">
	import type { PageData } from "./$types";

	export let data: PageData;
</script>
<div class="container mx-auto mt-16">
	<h1 class="h1 text-center">Todos</h1>

	<form action="http://0.0.0.0:8000/create" method="POST">
		<input class="input" name="description" type="text" placeholder="What needs to be done" />
	</form>

	{#each data.todos as todo}
	<p>{todo.id}</p>
	<p>{todo.description}</p>
	<p>{todo.done}</p>
	{/each}
</div>
