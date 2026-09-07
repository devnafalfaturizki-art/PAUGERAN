const layers = ['Gramatikal', 'Sistematis', 'Teleologis', 'Sosiologis', 'Historis', 'Komparatif', 'Kritis'];

export default function LayerDisplay() {
	return (
		<section aria-label="Tujuh lapisan penalaran">
			<div class="section-title"><h3>Lapisan penalaran</h3><span>07</span></div>
			<ol class="next-steps">
				{layers.map((layer, index) => <li><strong>{index + 1}. {layer}</strong><span> Menunggu input sumber.</span></li>)}
			</ol>
		</section>
	);
}
