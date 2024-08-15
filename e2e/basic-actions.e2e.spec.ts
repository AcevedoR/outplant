import { test, expect } from '@playwright/test';

test('displays introduction', async ({ page }) => {
  await page.goto('/');

  await expect(page).toHaveTitle(/Outplant/);
  await expect(page.getByText('Hello, I\'m Aude')).toBeVisible();
});

test('can make one choice', async ({ page }) => {
  await page.goto('/');

  await page.getByRole('button').click();
  await expect(page.getByRole('button').first(), 'you should be able to make a choice after introduction').toBeVisible();

  await page.getByRole('button').first().click();
  await expect(page.getByRole('button').first(), 'you should be able to continue to play after making one choice').toBeVisible();
});

test('a choice with a truthy condition should be displayed', async ({ page }) => {
  await page.goto(encodeURI('/?overrideInputChainFilenames=a_simple_empty_chain_with_an_hidden_choice'));

  await page.getByRole('button').click();
  await expect(page.getByRole('button').first(), 'you should be able to make a choice after introduction').toBeVisible();

  await expect(page.getByRole('button'), 'hidden choices shouldn\'t be available').toHaveCount(2);
  await expect(page.getByRole('button').filter({hasText: "a displayed choice"}).first()).toBeVisible();
  await expect(page.getByRole('button').filter({hasText: "a displayed choice because enough money"}).first()).toBeVisible();
});