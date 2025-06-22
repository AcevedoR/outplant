import { expect, type Page, test } from '@playwright/test';

test('displays introduction', async ({ page }) => {
	await page.goto('/');

	await expect(page).toHaveTitle(/Outplant/);
	await expect(page.getByText("Hello, I'm Aude")).toBeVisible();
});

test('can make one choice', async ({ page }) => {
	await page.goto('/');
	await startPlaying(page);

	await page.getByRole('button').first().click();
	await expect(
		page.getByRole('button').first(),
		'you should be able to continue to play after making one choice',
	).toBeVisible();
});

test('a choice with a truthy condition should be displayed', async ({
	page,
}) => {
	await page.goto(
		encodeURI(
			'/?overrideInputChainFilenames=a_simple_empty_chain_with_an_hidden_choice',
		),
	);
	await startPlaying(page);

	await expect(
		page.getByRole('button'),
		"hidden choices shouldn't be available",
	).toHaveCount(2);
	await expect(
		page.getByRole('button').filter({ hasText: 'a displayed choice' }).first(),
	).toBeVisible();
	await expect(
		page
			.getByRole('button')
			.filter({ hasText: 'a displayed choice because enough money' })
			.first(),
	).toBeVisible();
});

test('an outcome with a truthy condition should be displayed when condition is matched', async ({
	page,
}) => {
	const testedChain =
		'a_simple_chain_with_hidden_and_not_outcomes_conditioned_on_pop';
	await test.step(`Given ${testedChain}`, async () => {
		await page.goto(encodeURI(`/?overrideInputChainFilenames=${testedChain}`));
		await startPlaying(page);
	});

	await test.step('When there is 2 pop we should get the corresponding event', async () => {
		await expect(page.getByText('outcome_below_3_pop')).toBeVisible();
		await expect(
			page.getByText('outcome_above_and_eq_3_pop'),
		).not.toBeVisible();
	});

	await test.step('Wait a cycle to have 3 pop', async () => {
		await page.getByRole('button').click();
	});

	await test.step('When there is 3 pop we should get the corresponding event', async () => {
		await expect(page.getByText('outcome_below_3_pop')).not.toBeVisible();
		await expect(page.getByText('outcome_above_and_eq_3_pop')).toBeVisible();
	});
});

test('a variable should be able to be used as a condition', async ({
	page,
}) => {
	const testedChain = 'chain_with_variable';
	await test.step(`Given a ${testedChain}`, async () => {
		await page.goto(encodeURI(`/?overrideInputChainFilenames=${testedChain}`));
		await startPlaying(page);
	});

	await test.step('See if engine has selected the right outcome', async () => {
		await expect(page.getByText('IS OK')).toBeVisible();
	});
});

async function startPlaying(page: Page) {
	await page.getByRole('button').click();
	await expect(
		page.getByRole('button').first(),
		'you should be able to make a choice after introduction',
	).toBeVisible();
}
