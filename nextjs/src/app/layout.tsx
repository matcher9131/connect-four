import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Connect Four',
  description: 'Connect Four game powered by Next.js and Rust/WASM',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
